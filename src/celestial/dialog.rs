use native_dialog::DialogBuilder;

pub fn show_error_dialog(title: &str, message: &str) {
    DialogBuilder::message()
        .set_level(native_dialog::MessageLevel::Error)
        .set_title(title)
        .set_text(message)
        .alert()
        .show()
        .unwrap_or_default();
}
