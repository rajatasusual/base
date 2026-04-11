use tauri::{AppHandle, Manager};

mod utility;
use utility::normalize_or_search;

#[tauri::command]
fn open_url(app: AppHandle, input: &str) -> Result<(), String> {
    let url_str = normalize_or_search(input);

    let url = url_str
        .parse()
        .map_err(|e| format!("Invalid URL: {e}"))?;

    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    window.navigate(url).map_err(|e| format!("Failed to navigate: {e}"))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}