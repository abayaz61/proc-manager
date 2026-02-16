fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Process Manager");
        res.set("FileDescription", "High-performance process manager TUI");
        res.compile().expect("Failed to compile Windows resources");
    }
}
