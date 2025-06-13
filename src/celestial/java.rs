use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

use tokio::fs;

use crate::celestial::error::InternalError;

#[cfg(target_os = "windows")]
const JAVA_EXECUTABLE: &str = "javaw.exe";

#[cfg(not(target_os = "windows"))]
const JAVA_EXECUTABLE: &str = "java";

async fn is_java21(java_path: &PathBuf) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let output = tokio::process::Command::new(java_path)
        .arg("-version")
        .output()
        .await?;

    if output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(stderr.contains("version \"21") || stderr.contains("version 21"))
    } else {
        Ok(false)
    }
}

pub async fn find_java21() -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    if let Ok(java_home) = env::var("JAVA_HOME") {
        let candidate = PathBuf::from(java_home)
            .join("bin")
            .join(JAVA_EXECUTABLE);
        if fs::symlink_metadata(&candidate).await.is_ok() && is_java21(&candidate).await? {
            return Ok(candidate);
        }
    }

    let path_var = env::var("PATH").map_err(|_| Box::new(InternalError::new("No PATH env var")) as Box<dyn Error + Send + Sync>)?;
    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(JAVA_EXECUTABLE);
        if fs::symlink_metadata(&candidate).await.is_ok() && is_java21(&candidate).await? {
            return Ok(candidate);
        }
    }

    Err(Box::new(InternalError::new("No Java 21 found")))
}

pub async fn run_jar(java: &Path, jar_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let status = tokio::process::Command::new(java)
        .arg("-jar")
        .arg(jar_path)
        .status()
        .await?;

    if !status.success() {
        Err(format!("Java exits with code: {:?}", status.code()).into())
    } else {
        Ok(())
    }
}