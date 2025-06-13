use std::{path::PathBuf, sync::Arc};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

use crate::celestial::error::InternalError;
use crate::celestial::hash::sha256_file_async;



pub async fn download_file_with_progress(
    client: &Client,
    url: &str,
    path: &PathBuf,
    expect_sha256: Option<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // create dirs
    let Some(parent_dir) = path.parent() else {
        return Err(Box::new(InternalError::new(
            "No parent folder found",
        )));
    };

    if let Some(expect_sha256) = &expect_sha256 {
        if let Ok(hash) = sha256_file_async(path).await {
            if &hash == expect_sha256 {
                // hash matches, skip download
                return Ok(());
            }
        };
    }

    tokio::fs::create_dir_all(parent_dir).await?;
    println!("⬇️ Download to {}", path.display());

    let res = client.head(url).send().await?;
    let total_size = res
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    let mut file = tokio::fs::File::create_new(path).await?;
    let mut stream = client.get(url).send().await?.bytes_stream();

    let pb = Arc::new(Mutex::new(pb));

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        let pb = pb.lock().await;
        pb.inc(chunk.len() as u64);
    }

    // check sha256 after downloaded
    if let Some(expect_sha256) = expect_sha256 {
        let hash = sha256_file_async(path).await?;
        if hash != expect_sha256 {
            return Err(Box::new(InternalError::new(
                "Hash didn't match",
            )));
        }
    }

    pb.lock().await.finish_with_message("Download completed");
    Ok(())
}
