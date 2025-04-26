#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{env, io};
use dirs::home_dir;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

const GITHUB_API_URL: &str = "https://api.github.com/repos/earthsworth/celestial/releases/latest";

fn get_jar_dir() -> PathBuf {
    let mut path = home_dir().expect("Cannot get user home directory");
    path.push(".cubewhy");
    path.push("lunarcn");
    path.push("bootstrap");
    path
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // check Java 21
    let java_path = find_java21()?;

    // create HTTP client
    let client = Client::new();

    // fetch release info
    let release_info: serde_json::Value = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "Rust-Script")
        .send()
        .await?
        .json()
        .await?;

    let latest_version = release_info["tag_name"].as_str().unwrap_or("unknown");
    println!("最新版本: {}", latest_version);

    // 检查是否有可下载的 jar 文件
    if let Some(asset) = release_info["assets"].as_array().and_then(|assets| {
        assets
            .iter()
            .find(|a| a["name"].as_str().unwrap_or("").ends_with(".jar"))
    }) {
        let jar_url = asset["browser_download_url"].as_str().unwrap();
        let jar_name = asset["name"].as_str().unwrap();

        let jar_dir = get_jar_dir();
        let jar_path = jar_dir.join(jar_name);

        fs::create_dir_all(&jar_dir)?;

        // 检查文件是否已存在
        if !Path::new(&jar_path).exists() {
            println!("下载 {}...", jar_name);
            download_file_with_progress(&client, jar_url, &jar_path).await?;
        } else {
            println!("文件已存在，跳过下载");
        }

        // 使用 Java 21 运行 jar
        println!("使用 Java 21 启动 {}...", jar_name);
        let status = std::process::Command::new(&java_path)
            .arg("-jar")
            .arg(&jar_path)
            .status()?;

        if !status.success() {
            return Err(format!("运行 jar 文件失败: {}", status).into());
        }
    } else {
        return Err("未找到可下载的 jar 文件".into());
    }

    Ok(())
}

async fn download_file_with_progress(
    client: &Client,
    url: &str,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // send HEAD request in order to know the file size
    let res = client.head(url).send().await?;
    let total_size = res
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0);

    // Create progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    // Download celestial
    let mut file = tokio::fs::File::create(path).await?;
    let mut stream = client.get(url).send().await?.bytes_stream();

    let pb = Arc::new(Mutex::new(pb));

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        let pb = pb.lock().await;
        pb.inc(chunk.len() as u64);
    }

    pb.lock().await.finish_with_message("下载完成");

    Ok(())
}

fn find_java21() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Check JAVA_HOME
    if let Ok(java_home) = env::var("JAVA_HOME") {
        let java_path =
            Path::new(&java_home)
                .join("bin")
                .join(if cfg!(windows) { "javaw.exe" } else { "java" });
        if java_path.exists() && check_java_version(&java_path)? {
            return Ok(java_path);
        }
    }

    // Check java in PATH
    if let Ok(path) = find_in_path(if cfg!(windows) { "javaw.exe" } else { "java" }) {
        if check_java_version(&path)? {
            return Ok(path.to_path_buf());
        }
    }

    // No java21
    if cfg!(windows) {
        println!("未找到 Java 21，Celestial Bootstrap会尝试帮你下载一个, 如果失败请访问下方链接手动下载.");
        println!("https://azul.com");
    } else {
        println!("未找到 Java 21，请使用系统包管理器安装:");
        println!("例如:");
        println!("  Ubuntu/Debian: sudo apt install openjdk-21-jdk");
        println!("  Fedora/RHEL: sudo dnf install java-21-openjdk");
        println!("  Arch Linux: sudo pacman -S jdk-openjdk");
        println!("  macOS (Homebrew): brew install openjdk@21");
    }

    Err("Java 21 未安装".into())
}

fn find_in_path(executable: &str) -> io::Result<PathBuf> {
    if let Ok(path) = env::var("PATH") {
        for dir in env::split_paths(&path) {
            let full_path = dir.join(executable);
            if full_path.exists() {
                return Ok(full_path);
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "未找到可执行文件"))
}

fn check_java_version(java_path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let output = Command::new(java_path)
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        let version_output = String::from_utf8(output.stderr)?;
        // Check java version is 21
        if version_output.contains("version \"21") || version_output.contains("version 21") {
            return Ok(true);
        }
    }
    Ok(false)
}
