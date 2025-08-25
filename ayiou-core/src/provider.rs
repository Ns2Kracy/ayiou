use std::io::{Read, Write};
use std::path::Path;

use async_trait::async_trait;

use crate::{error::AyiouError, file_info::FileInfo};

#[async_trait]
pub trait Provider: Send + Sync {
    async fn list_dir(&self, path: &Path) -> Result<Vec<FileInfo>, AyiouError>;
    async fn read_file(&self, path: &Path) -> Result<Box<dyn Read + Send>, AyiouError>;
    async fn write_file(&mut self, path: &Path, data: &mut dyn Write) -> Result<(), AyiouError>;
    async fn create_file(&mut self, path: &Path) -> Result<(), AyiouError>;
    async fn delete_file(&mut self, path: &Path) -> Result<(), AyiouError>;
    async fn mkdir(&mut self, path: &Path) -> Result<(), AyiouError>;
    async fn rmdir(&mut self, path: &Path) -> Result<(), AyiouError>;
}
