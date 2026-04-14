use tauri::Emitter;
use tauri_plugin_window_state::{Builder as WindowBuilder, StateFlags};

mod commands;
mod utility;

#[derive(Clone, serde::Serialize)]
struct SingleInstancePayload {
    args: Vec<String>,
    cwd: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit("single-instance", SingleInstancePayload { args: argv, cwd })
                .unwrap();
        }))
        .plugin(
            WindowBuilder::default()
                .with_state_flags(StateFlags::all() & !StateFlags::DECORATIONS)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::transcribe::transcribe,
            commands::answer::answer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
