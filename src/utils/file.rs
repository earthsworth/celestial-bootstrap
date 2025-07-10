use std::path::Path;

use tokio::fs::{File, OpenOptions};

pub async fn safe_create_or_open(path: &Path) -> std::io::Result<File> {
    match File::create_new(path).await {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            // fallback: open with write (not just read)
            OpenOptions::new()
                .write(true)
                .read(true)
                .open(path)
                .await
        }
        Err(e) => Err(e),
    }
}
