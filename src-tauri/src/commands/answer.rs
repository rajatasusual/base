use std::ffi::OsString;

use crate::utility::{
    command_for, configured_model_name, resolve_binary, resolve_model, third_party_dir,
};

#[tauri::command]
pub fn answer(app: tauri::AppHandle, prompt: String) -> Result<String, String> {
    if prompt.len() > 4096 {
        return Err("Prompt too long".to_string());
    }

    let third_party = third_party_dir(&app);
    let llama_cli = resolve_binary(
        &third_party,
        "llama.cpp",
        &["llama-completion", "llama-cli"],
    )?;
    let mut model_names = Vec::new();

    if let Some(model_name) = configured_model_name(&third_party, "LLAMA_MODEL_NAME", "gguf") {
        model_names.push(model_name);
    }

    model_names.extend([
        OsString::from("smollm2-360m-instruct-q8_0.gguf")
    ]);

    let model = resolve_model(&third_party, "llama.cpp", &model_names)?;

    let output = command_for(&llama_cli)
        .arg("-m")
        .arg(&model)
        .arg("--cpu-strict")
        .arg("1")
        .arg("-t")
        .arg("2")
        .arg("-n")
        .arg("256")
        .arg("--top-k")
        .arg("2")
        .arg("-p")
        .arg(&prompt)
        .arg("-no-cnv")
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
