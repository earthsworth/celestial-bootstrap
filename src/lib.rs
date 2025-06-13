use std::env::home_dir;

use crate::celestial::{
    java::{find_java21, run_jar},
    updater::check_update,
};

pub mod celestial;

use celestial::error::InternalError;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Welcome to Celestial, the FOSS LunarClient Launcher implementation!");
    println!(
        "💡 Celestial is completely free and open source software and if you bought Celestial anywhere, you got ripped off!"
    );
    println!("💡 Source code is available at https://git.lunarclient.top/CubeWhyMC");
    println!("♥️ Donate us at https://lunarclient.top/donate");
    println!("💬 Join our Discord! https://discord.lunarclient.top");
    println!("✈️ Join our Telegram! https://t.me/earthsworth");

    let Some(home) = home_dir() else {
        eprintln!("No home dir detected");
        return Err(Box::new(InternalError::new("No home dir detected")));
    };
    let home: std::path::PathBuf = home.join(".cubewhy/lunarcn");
    let celestial_jar_path = &home.join(".bootstrap/celestial.jar");

    // check update
    check_update(&home, celestial_jar_path)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Err! {err}");
        });

    // launch celestial
    let Ok(java_executables) = find_java21().await else {
        eprintln!(
            "Since Celestial bootstrap doesn't have the feature of downloading Java 21, please download manually"
        );
        eprintln!("https://www.azul.com/downloads/?version=java-21-lts&package=jdk#zulu");
        return Err(Box::new(InternalError::new("No java 21 found, please download one. Since Celestial bootstrap doesn't have the feature of downloading Java 21, please download manually. https://www.azul.com/downloads/?version=java-21-lts&package=jdk#zulu
        ")));
    };
    run_jar(&java_executables, &celestial_jar_path).await?;

    Ok(())
}
