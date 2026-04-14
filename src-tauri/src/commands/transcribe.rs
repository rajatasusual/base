use std::io::Write;
use std::os::windows::process::CommandExt;
use std::process::Command;
use tempfile::Builder;

use crate::utility::{project_root, CREATE_NO_WINDOW};

#[tauri::command]
pub fn transcribe(wav: Vec<u8>) -> Result<String, String> {
    const MAX_WAV_BYTES: usize = 100 * 1024 * 1024; // 100 MB
    if wav.len() > MAX_WAV_BYTES {
        return Err(format!(
            "WAV too large: {} bytes (max {})",
            wav.len(),
            MAX_WAV_BYTES
        ));
    }
    
    let root = project_root();
    let whisper_cli = root.join("third-party/whisper.cpp/whisper-cli.exe");
    let model = root.join("third-party/whisper.cpp/model/ggml-base.en.bin");

    if !whisper_cli.exists() {
        return Err(format!(
            "whisper-cli not found at: {}",
            whisper_cli.display()
        ));
    }
    if !model.exists() {
        return Err(format!("model not found at: {}", model.display()));
    }

    let tmp_path = {
        let mut tmp = Builder::new()
            .suffix(".wav")
            .tempfile()
            .map_err(|e| e.to_string())?;
        tmp.write_all(&wav).map_err(|e| e.to_string())?;
        tmp.flush().map_err(|e| e.to_string())?;
        tmp.into_temp_path()
    };

    let output = Command::new(&whisper_cli)
        .args([
            "-m",
            model.to_str().unwrap(),
            "-f",
            tmp_path.to_str().unwrap(),
            "--no-timestamps",
        ])
        .creation_flags(CREATE_NO_WINDOW)
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
