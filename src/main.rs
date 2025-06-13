use celestial_bootstrap::celestial::dialog::show_error_dialog;

#[tokio::main]
async fn main() {
    celestial_bootstrap::run().await.unwrap_or_else(|err| {
        show_error_dialog("Celestial Bootstrap - Error", err.to_string().as_str());
        eprintln!("❌ Error: {err}");
    });
}
