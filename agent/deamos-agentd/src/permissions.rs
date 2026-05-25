//! Permission model for tool actions.
//!
//! Two independent layers:
//!   1. **Approver** — the decision point. The interactive omnibox prompt will
//!      implement this trait; until then the safe default denies.
//!   2. **FsPolicy** — hard guardrails enforced regardless of the approver
//!      (permitted roots, traversal neutralization).

use std::path::{Path, PathBuf};

/// A request to perform a tool action, handed to an [`Approver`].
#[derive(Debug, Clone)]
pub struct PermissionRequest {
    pub tool: String,
    pub action: String,
    pub resource: String,
}

/// The approver's verdict.
#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny(String),
}

/// Decides whether a tool action may proceed. The future interactive UI
/// approver implements this same trait.
pub trait Approver: Send + Sync {
    fn decide(&self, request: &PermissionRequest) -> Decision;
}

/// Skeleton default: denies anything needing approval, because no interactive
/// UI approver is attached yet. Safe placeholder for the omnibox prompt.
pub struct RequireInteractiveApproval;

impl Approver for RequireInteractiveApproval {
    fn decide(&self, _request: &PermissionRequest) -> Decision {
        Decision::Deny("interactive user approval required".to_string())
    }
}

/// Approves every request, deferring all enforcement to policy guardrails.
/// Used for headless/testing or when `require_approval` is disabled.
pub struct AutoApprover;

impl Approver for AutoApprover {
    fn decide(&self, _request: &PermissionRequest) -> Decision {
        Decision::Allow
    }
}

/// The live approval path: ask the user via a GUI dialog (kdialog, then
/// zenity). Denies if the user declines or no GUI helper is available. Used by
/// the daemon when `require_approval` is on — this is what makes the omnibox
/// permission prompt real.
pub struct InteractiveApprover;

impl Approver for InteractiveApprover {
    fn decide(&self, request: &PermissionRequest) -> Decision {
        let prompt = format!(
            "Deamos Agent wants to {} {}.\n\nAllow this?",
            request.action, request.resource
        );
        match gui_confirm(&prompt) {
            Some(true) => Decision::Allow,
            Some(false) => Decision::Deny("user declined".to_string()),
            None => Decision::Deny("no GUI prompt available".to_string()),
        }
    }
}

/// Ask a yes/no question through whatever GUI dialog tool is present.
/// `Some(true/false)` = user answered; `None` = no dialog tool found.
fn gui_confirm(prompt: &str) -> Option<bool> {
    let candidates: [(&str, Vec<&str>); 2] = [
        ("kdialog", vec!["--warningyesno", prompt]),
        ("zenity", vec!["--question", "--text", prompt]),
    ];
    for (bin, args) in candidates {
        // Ok = the tool ran (exit 0 = yes); Err = the tool is not installed.
        if let Ok(status) = std::process::Command::new(bin).args(&args).status() {
            return Some(status.success());
        }
    }
    None
}

/// Reason a filesystem access was refused by policy.
#[derive(Debug)]
pub struct Denied {
    pub reason: String,
}

/// Hard guardrails for filesystem tool access.
pub struct FsPolicy {
    /// Reads are only permitted under these (canonicalized) roots.
    pub allowed_roots: Vec<PathBuf>,
    /// Largest single read permitted, in bytes.
    pub max_read_bytes: u64,
}

impl FsPolicy {
    /// Resolve and authorize a read. Canonicalization neutralizes `..`
    /// traversal and symlinks; the result must sit under a permitted root.
    pub fn authorize_read(&self, requested: &Path) -> Result<PathBuf, Denied> {
        let canonical = std::fs::canonicalize(requested).map_err(|e| Denied {
            reason: format!("cannot resolve path: {e}"),
        })?;

        if self.allowed_roots.is_empty() {
            return Err(Denied {
                reason: "no filesystem roots are permitted".to_string(),
            });
        }

        let permitted = self.allowed_roots.iter().any(|root| {
            std::fs::canonicalize(root)
                .map(|r| canonical.starts_with(&r))
                .unwrap_or(false)
        });

        if permitted {
            Ok(canonical)
        } else {
            Err(Denied {
                reason: "path is outside permitted roots".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp_dir(tag: &str) -> PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = std::env::temp_dir().join(format!("deamos-{tag}-{n}"));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn allows_inside_root_denies_outside() {
        let root = tmp_dir("perm");
        let file = root.join("ok.txt");
        std::fs::write(&file, b"hi").unwrap();
        let policy = FsPolicy {
            allowed_roots: vec![root.clone()],
            max_read_bytes: 1024,
        };
        assert!(policy.authorize_read(&file).is_ok());

        // The temp dir itself sits above the permitted root → denied.
        assert!(policy.authorize_read(&std::env::temp_dir()).is_err());
    }

    #[test]
    fn empty_roots_denies_everything() {
        let root = tmp_dir("perm");
        let file = root.join("x.txt");
        std::fs::write(&file, b"x").unwrap();
        let policy = FsPolicy {
            allowed_roots: vec![],
            max_read_bytes: 1024,
        };
        assert!(policy.authorize_read(&file).is_err());
    }

    #[test]
    fn approvers_behave() {
        let req = PermissionRequest {
            tool: "fs".to_string(),
            action: "read".to_string(),
            resource: "/x".to_string(),
        };
        assert_eq!(AutoApprover.decide(&req), Decision::Allow);
        assert!(matches!(
            RequireInteractiveApproval.decide(&req),
            Decision::Deny(_)
        ));
    }
}
