use std::path::{Path, PathBuf};

use reqwest::Client;

use crate::celestial::downloader::download_file_with_progress;

pub async fn check_update(
    base_dir: &Path,
    celestial_jar_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let is_first_run = !celestial_jar_file.exists();
    // Note: since Celestial is disconnected, so this function only do verify the file hash
    let client = Client::new();
    // download Celestial
    download_file_with_progress(
        &client,
        "https://lunarclient.top/dl/celestial-3.2.1-hotfix.jar",
        celestial_jar_file,
        Some(String::from(
            "382003745e4fc6e34a5f1bd1096468574a89739ba88a9d6465b97edba97b1d30",
        )),
    )
    .await?; // TODO check errors

    // download lunar debugger (if is first run)
    if is_first_run {
        println!("💡 Seems it's your first time use Celestial, downloading LunarDebugger...");
        download_file_with_progress(
            &client,
            "https://lunarclient.top/dl/LunarDebugger-fatjar.jar",
            &base_dir.join("javaagents/LunarDebugger.jar"),
            Some(String::from(
                "9f99d23eae80c96871341c315b79efedbba9d64d3584e2434109f9110de59f8d",
            )),
        ).await?;
    }

    Ok(())
}
