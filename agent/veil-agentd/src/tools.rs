//! Agent tools. The first one — `fs_read` — reads a file after the [`FsPolicy`]
//! guardrails authorize it. More tools (terminal, browser, window) follow the
//! same shape: authorize, then act.

use serde::Serialize;
use std::path::Path;

use crate::permissions::FsPolicy;

/// Successful filesystem read.
#[derive(Debug, Serialize)]
pub struct FsReadResponse {
    pub path: String,
    pub bytes: u64,
    pub content: String,
}

/// Why a tool call failed.
#[derive(Debug)]
pub enum ToolError {
    Denied(String),
    NotAFile,
    TooLarge { max: u64, actual: u64 },
    Io(String),
}

/// Read a file, subject to policy: it must resolve under a permitted root and
/// be a regular file no larger than the configured limit.
pub fn fs_read(policy: &FsPolicy, requested: &Path) -> Result<FsReadResponse, ToolError> {
    let path = policy
        .authorize_read(requested)
        .map_err(|d| ToolError::Denied(d.reason))?;

    let meta = std::fs::metadata(&path).map_err(|e| ToolError::Io(e.to_string()))?;
    if !meta.is_file() {
        return Err(ToolError::NotAFile);
    }
    if meta.len() > policy.max_read_bytes {
        return Err(ToolError::TooLarge {
            max: policy.max_read_bytes,
            actual: meta.len(),
        });
    }

    let bytes = std::fs::read(&path).map_err(|e| ToolError::Io(e.to_string()))?;
    let content = String::from_utf8_lossy(&bytes).into_owned();
    Ok(FsReadResponse {
        path: path.display().to_string(),
        bytes: bytes.len() as u64,
        content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp_dir() -> PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d = std::env::temp_dir().join(format!("veil-tool-{n}"));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn reads_file_within_policy() {
        let root = tmp_dir();
        let f = root.join("hello.txt");
        std::fs::write(&f, b"hello veil").unwrap();
        let policy = FsPolicy {
            allowed_roots: vec![root],
            max_read_bytes: 1024,
        };
        let resp = fs_read(&policy, &f).unwrap();
        assert_eq!(resp.content, "hello veil");
        assert_eq!(resp.bytes, 10);
    }

    #[test]
    fn rejects_oversized_file() {
        let root = tmp_dir();
        let f = root.join("big.bin");
        std::fs::write(&f, vec![0u8; 2048]).unwrap();
        let policy = FsPolicy {
            allowed_roots: vec![root],
            max_read_bytes: 1024,
        };
        assert!(matches!(
            fs_read(&policy, &f),
            Err(ToolError::TooLarge { .. })
        ));
    }

    #[test]
    fn rejects_directory() {
        let root = tmp_dir();
        let policy = FsPolicy {
            allowed_roots: vec![root.clone()],
            max_read_bytes: 1024,
        };
        assert!(matches!(fs_read(&policy, &root), Err(ToolError::NotAFile)));
    }

    #[test]
    fn rejects_path_outside_root() {
        let root = tmp_dir();
        let policy = FsPolicy {
            allowed_roots: vec![root],
            max_read_bytes: 1024,
        };
        // temp_dir is above the root → Denied
        assert!(matches!(
            fs_read(&policy, &std::env::temp_dir()),
            Err(ToolError::Denied(_))
        ));
    }
}
