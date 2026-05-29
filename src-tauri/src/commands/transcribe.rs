use std::ffi::OsString;
use std::io::Write;
use tempfile::Builder;

use crate::utility::{
    command_for, configured_model_name, resolve_binary, resolve_model, third_party_dir,
};

#[tauri::command]
pub fn transcribe(app: tauri::AppHandle,wav: Vec<u8>) -> Result<String, String> {
    
    const MAX_WAV_BYTES: usize = 100 * 1024 * 1024; // 100 MB
    if wav.len() > MAX_WAV_BYTES {
        return Err(format!(
            "WAV too large: {} bytes (max {})",
            wav.len(),
            MAX_WAV_BYTES
        ));
    }

    let third_party = third_party_dir(&app);
    let whisper_cli = resolve_binary(&third_party, "whisper.cpp", &["whisper-cli", "main"])?;
    let mut model_names = Vec::new();

    if let Some(model_name) = configured_model_name(&third_party, "WHISPER_MODEL_NAME", "bin") {
        model_names.push(model_name);
    }

    model_names.push(OsString::from("ggml-base.en.bin"));

    let model = resolve_model(&third_party, "whisper.cpp", &model_names)?;

    let tmp_path = {
        let mut tmp = Builder::new()
            .suffix(".wav")
            .tempfile()
            .map_err(|e| e.to_string())?;
        tmp.write_all(&wav).map_err(|e| e.to_string())?;
        tmp.flush().map_err(|e| e.to_string())?;
        tmp.into_temp_path()
    };

    let output = command_for(&whisper_cli)
        .arg("-m")
        .arg(&model)
        .arg("-f")
        .arg(tmp_path.as_os_str())
        .arg("--no-timestamps")
        .output()
        .map_err(|e| {
            format!(
                "Failed to run whisper-cli: {e}\nPath: {}",
                whisper_cli.display()
            )
        })?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let transcript = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();

    Ok(transcript)
}
