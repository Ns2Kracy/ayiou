use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub modified: Option<DateTime<Utc>>,
    pub created: Option<DateTime<Utc>>,
    #[cfg(target_os = "windows")]
    pub last_accessed: Option<DateTime<Utc>>,
    pub permissions: u32,
    pub owner: Option<String>,
    pub group: Option<String>,
    #[cfg(target_os = "windows")]
    pub attributes: u32,
    #[cfg(target_os = "windows")]
    pub acl: Option<Vec<u8>>,
}

impl FileInfo {
    /// Returns the canonicalized path, resolving symlinks and normalizing separators.
    /// Falls back to the original path if canonicalization fails.
    pub fn canonical_path(&self) -> PathBuf {
        self.path
            .canonicalize()
            .unwrap_or_else(|_| self.path.clone())
    }
}
