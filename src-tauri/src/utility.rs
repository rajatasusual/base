pub const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn project_root() -> std::path::PathBuf {
    // In dev: resolves to src-tauri/  → go up one level to project root
    // In release: resolves relative to the .exe location
    if cfg!(debug_assertions) {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("src-tauri")
            .to_path_buf()
    } else {
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }
}