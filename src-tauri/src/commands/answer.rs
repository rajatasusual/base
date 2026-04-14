use std::os::windows::process::CommandExt;
use std::process::Command;

use crate::utility::{project_root, CREATE_NO_WINDOW};

#[tauri::command]
pub fn answer(prompt: String) -> Result<String, String> {
    if prompt.len() > 4096 {
        return Err("Prompt too long".to_string());
    }
    let root = project_root();
    let llama_cli = root.join("third-party/llama.cpp/llama-completion.exe");
    let model = root.join("third-party/llama.cpp/model/gemma3-270m-it.gguf");

    if !llama_cli.exists() {
        return Err(format!("llama-cli not found at: {}", llama_cli.display()));
    }
    if !model.exists() {
        return Err(format!("model not found at: {}", model.display()));
    }

    let output = Command::new(&llama_cli)
        .args([
            "-m",
            model.to_str().unwrap(),
            "--cpu-strict",
            "1",
            "-t",
            "0",
            "-n",
            "128",
            "-p",
            &prompt,
            "-no-cnv",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run llama-cli: {e}\nPath: {}",
                llama_cli.display()
            )
        })?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let answer = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    Ok(answer)
}
