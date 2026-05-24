//! The agent loop: hand the model a prompt plus the available tools, let it
//! call them, enforce permissions on every call, feed results back, and return
//! the final answer. This is where the LLM, the tools, and the permission gate
//! meet — the core of Veil Agent.

use serde::Serialize;
use std::path::Path;
use std::sync::Arc;

use crate::llm::{ChatBackend, FunctionDef, LlmError, Message, ToolCall, ToolDef};
use crate::permissions::{Approver, Decision, FsPolicy, PermissionRequest};
use crate::tools::{fs_read, terminal_exec};

/// How many tool round-trips before we give up.
const MAX_STEPS: usize = 5;
/// Per-stream cap on captured command output.
const MAX_EXEC_OUTPUT: usize = 16 * 1024;

/// A record of one tool invocation, returned for transparency.
#[derive(Debug, Serialize)]
pub struct ToolStep {
    pub tool: String,
    pub argument: String,
    pub outcome: String,
}

/// The result of an agent run.
#[derive(Debug, Serialize)]
pub struct AgentOutcome {
    pub answer: String,
    pub steps: Vec<ToolStep>,
}

/// Orchestrates one conversation, including tool calls.
pub struct Agent {
    backend: Arc<dyn ChatBackend>,
    model: String,
    fs_policy: Arc<FsPolicy>,
    approver: Arc<dyn Approver>,
    enable_terminal: bool,
}

impl Agent {
    pub fn new(
        backend: Arc<dyn ChatBackend>,
        model: String,
        fs_policy: Arc<FsPolicy>,
        approver: Arc<dyn Approver>,
        enable_terminal: bool,
    ) -> Self {
        Self {
            backend,
            model,
            fs_policy,
            approver,
            enable_terminal,
        }
    }

    /// Tools advertised to the model. `terminal_exec` is only offered when
    /// enabled in config.
    fn tools(&self) -> Vec<ToolDef> {
        let mut tools = vec![ToolDef {
            kind: "function".to_string(),
            function: FunctionDef {
                name: "fs_read".to_string(),
                description: "Read a UTF-8 text file from a permitted directory.".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Path to the file to read" }
                    },
                    "required": ["path"]
                }),
            },
        }];
        if self.enable_terminal {
            tools.push(ToolDef {
                kind: "function".to_string(),
                function: FunctionDef {
                    name: "terminal_exec".to_string(),
                    description:
                        "Run a shell command and return its output. Requires user approval."
                            .to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "command": { "type": "string", "description": "Shell command to run" }
                        },
                        "required": ["command"]
                    }),
                },
            });
        }
        tools
    }

    /// Run the loop until the model answers without calling a tool, or the step
    /// budget is exhausted.
    pub async fn run(&self, user_prompt: &str) -> Result<AgentOutcome, LlmError> {
        let tools = self.tools();
        let mut messages = vec![
            Message::system("You are Veil Agent. Use the provided tools when helpful. Be concise."),
            Message::user(user_prompt),
        ];
        let mut steps = Vec::new();

        for _ in 0..MAX_STEPS {
            let reply = self.backend.chat(&self.model, &messages, &tools).await?;
            let tool_calls = reply.tool_calls.clone().unwrap_or_default();

            if tool_calls.is_empty() {
                return Ok(AgentOutcome {
                    answer: reply.content,
                    steps,
                });
            }

            // Keep the assistant's tool-call message in the history.
            messages.push(reply);

            for call in tool_calls {
                let (result, step) = self.execute(&call);
                steps.push(step);
                messages.push(Message::tool(&call.function.name, result));
            }
        }

        Ok(AgentOutcome {
            answer: "Stopped: reached the maximum number of tool steps.".to_string(),
            steps,
        })
    }

    /// Execute one tool call, enforcing the permission gate and policy.
    /// Returns the text fed back to the model and a step record.
    fn execute(&self, call: &ToolCall) -> (String, ToolStep) {
        match call.function.name.as_str() {
            "fs_read" => self.exec_fs_read(call),
            "terminal_exec" => self.exec_terminal(call),
            other => {
                let outcome = format!("error: unknown tool {other}");
                (
                    outcome.clone(),
                    ToolStep {
                        tool: other.to_string(),
                        argument: String::new(),
                        outcome,
                    },
                )
            }
        }
    }

    fn exec_fs_read(&self, call: &ToolCall) -> (String, ToolStep) {
        let path = arg_str(call, "path");
        if let Some(reason) = self.gate("fs", "read", &path) {
            return tool_denied("fs_read", &path, &reason);
        }
        match fs_read(&self.fs_policy, Path::new(&path)) {
            Ok(resp) => (resp.content, tool_ok("fs_read", &path)),
            Err(e) => tool_error("fs_read", &path, format!("error: {e:?}")),
        }
    }

    fn exec_terminal(&self, call: &ToolCall) -> (String, ToolStep) {
        let command = arg_str(call, "command");
        if !self.enable_terminal {
            return tool_denied(
                "terminal_exec",
                &command,
                "terminal is disabled (enable_terminal=false)",
            );
        }
        if let Some(reason) = self.gate("terminal", "run", &command) {
            return tool_denied("terminal_exec", &command, &reason);
        }
        match terminal_exec(&command, MAX_EXEC_OUTPUT) {
            Ok(r) => (
                format!(
                    "exit {}\nstdout:\n{}\nstderr:\n{}",
                    r.exit_code, r.stdout, r.stderr
                ),
                tool_ok("terminal_exec", &command),
            ),
            Err(e) => tool_error("terminal_exec", &command, format!("error: {e:?}")),
        }
    }

    /// Consult the approver. Returns the deny reason, or `None` if allowed.
    fn gate(&self, tool: &str, action: &str, resource: &str) -> Option<String> {
        let request = PermissionRequest {
            tool: tool.to_string(),
            action: action.to_string(),
            resource: resource.to_string(),
        };
        match self.approver.decide(&request) {
            Decision::Deny(reason) => Some(reason),
            Decision::Allow => None,
        }
    }
}

fn arg_str(call: &ToolCall, key: &str) -> String {
    call.function
        .arguments
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

fn tool_ok(tool: &str, argument: &str) -> ToolStep {
    ToolStep {
        tool: tool.to_string(),
        argument: argument.to_string(),
        outcome: "ok".to_string(),
    }
}

fn tool_denied(tool: &str, argument: &str, reason: &str) -> (String, ToolStep) {
    let outcome = format!("denied: {reason}");
    (
        outcome.clone(),
        ToolStep {
            tool: tool.to_string(),
            argument: argument.to_string(),
            outcome,
        },
    )
}

fn tool_error(tool: &str, argument: &str, message: String) -> (String, ToolStep) {
    (
        message.clone(),
        ToolStep {
            tool: tool.to_string(),
            argument: argument.to_string(),
            outcome: message,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::FunctionCall;
    use crate::permissions::AutoApprover;
    use async_trait::async_trait;
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// Backend that replays a scripted sequence of assistant messages.
    struct ScriptedBackend {
        steps: Mutex<VecDeque<Message>>,
    }

    #[async_trait]
    impl ChatBackend for ScriptedBackend {
        async fn generate(&self, _model: &str, _prompt: &str) -> Result<String, LlmError> {
            Ok(String::new())
        }
        async fn chat(
            &self,
            _model: &str,
            _messages: &[Message],
            _tools: &[ToolDef],
        ) -> Result<Message, LlmError> {
            Ok(self
                .steps
                .lock()
                .unwrap()
                .pop_front()
                .expect("scripted backend ran out of steps"))
        }
    }

    fn assistant_tool_call(path: &str) -> Message {
        Message {
            role: "assistant".to_string(),
            content: String::new(),
            tool_calls: Some(vec![ToolCall {
                function: FunctionCall {
                    name: "fs_read".to_string(),
                    arguments: serde_json::json!({ "path": path }),
                },
            }]),
            tool_name: None,
        }
    }

    #[tokio::test]
    async fn agent_calls_tool_then_answers() {
        // Arrange a readable file inside a permitted root.
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("veil-agent-{n}"));
        std::fs::create_dir_all(&root).unwrap();
        let file = root.join("note.txt");
        std::fs::write(&file, b"the secret is 42").unwrap();

        // Script: first the model calls fs_read, then it answers.
        let mut q = VecDeque::new();
        q.push_back(assistant_tool_call(file.to_str().unwrap()));
        q.push_back(Message::system("The file says the secret is 42."));
        let backend = Arc::new(ScriptedBackend {
            steps: Mutex::new(q),
        });

        let policy = Arc::new(FsPolicy {
            allowed_roots: vec![root],
            max_read_bytes: 1024,
        });
        let agent = Agent::new(
            backend,
            "test".to_string(),
            policy,
            Arc::new(AutoApprover),
            false,
        );

        let outcome = agent.run("what is the secret?").await.unwrap();
        assert_eq!(outcome.steps.len(), 1);
        assert_eq!(outcome.steps[0].tool, "fs_read");
        assert_eq!(outcome.steps[0].outcome, "ok");
        assert!(outcome.answer.contains("42"));
    }
}
