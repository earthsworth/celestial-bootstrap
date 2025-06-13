use std::{io, path::PathBuf};
use sha2::{Sha256, Digest};

use tokio::{fs::File, io::AsyncReadExt};

pub async fn sha256_file_async(path: &PathBuf) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let hash_result = hasher.finalize();
    Ok(format!("{:x}", hash_result))
}