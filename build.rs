fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico")
            .set("ProductName", "Celestial")
            .set("FileDescription", "Celestial Bootstrap")
            .set("LegalCopyright", "Copyright (c) 2025 earthsworth");
        res.compile().unwrap();
    }
}
