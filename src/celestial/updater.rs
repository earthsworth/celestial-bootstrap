use std::path::PathBuf;

use reqwest::Client;

use crate::celestial::downloader::download_file_with_progress;

pub async fn check_update(
    base_dir: &PathBuf,
    celestial_jar_file: &PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let is_first_run = !celestial_jar_file.exists();
    // Note: since Celestial is disconnected, so this function only do verify the file hash
    let client = Client::new();
    // download Celestial
    download_file_with_progress(
        &client,
        "https://lunarclient.top/dl/celestial-3.2.1-SNAPSHOT-fatjar.jar",
        celestial_jar_file,
        Some(String::from(
            "561beb82c97f03efd25b57f502be20a1ffec8ec87c8345bfcc07ad6c0573e678",
        )),
    )
    .await?; // TODO check errors

    // download lunar debugger (if is first run)
    if is_first_run {
        println!("💡 Seems this is your first time use Celestial, downloading LunarDebugger...");
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
