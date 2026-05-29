use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use tauri::Manager;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn third_party_dir(app: &tauri::AppHandle) -> std::path::PathBuf {
    if cfg!(debug_assertions) {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("third-party")
    } else {
        app.path()
            .resource_dir()
            .expect("failed to get resource dir")
            .join("third-party")
    }
}

pub fn resolve_binary(
    third_party_root: &Path,
    project_dir: &str,
    binary_names: &[&str],
) -> Result<PathBuf, String> {
    let project_root = third_party_root.join(project_dir);

    for binary_name in binary_names {
        let executable_name = executable_name(binary_name);

        let candidates = [
            project_root.join("bin").join(&executable_name),
            project_root.join(&executable_name),
        ];

        for candidate in candidates {
            if candidate.is_file() {
                return Ok(candidate);
            }
        }

        if let Some(found) = find_file_by_name(&project_root.join("bin"), &executable_name) {
            return Ok(found);
        }
    }

    let expected = binary_names
        .iter()
        .map(|name| project_root.join("bin").join(executable_name(name)).display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Err(format!("Binary not found. Expected one of: {expected}"))
}

pub fn resolve_model(
    third_party_root: &Path,
    project_dir: &str,
    model_names: &[OsString],
) -> Result<PathBuf, String> {
    let model_dir = third_party_root.join(project_dir).join("model");

    for model_name in model_names {
        let candidate = model_dir.join(model_name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    let expected = model_names
        .iter()
        .map(|name| model_dir.join(name).display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Err(format!("Model not found. Expected one of: {expected}"))
}

pub fn configured_value(third_party_root: &Path, key: &str) -> Option<String> {
    let config = fs::read_to_string(third_party_root.join("versions.conf")).ok()?;

    config.lines().find_map(|line| {
        let line = line.trim();
        let (line_key, raw_value) = line.split_once('=')?;

        if line_key.trim() != key {
            return None;
        }

        Some(
            raw_value
                .split('#')
                .next()
                .unwrap_or_default()
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string(),
        )
    })
}

pub fn configured_model_name(
    third_party_root: &Path,
    key: &str,
    default_extension: &str,
) -> Option<OsString> {
    let mut name = configured_value(third_party_root, key)?;

    if Path::new(&name).extension().is_none() {
        name.push('.');
        name.push_str(default_extension);
    }

    Some(OsString::from(name))
}

pub fn command_for(binary_path: &Path) -> Command {
    let mut command = Command::new(binary_path);

    if let Some(parent) = binary_path.parent() {
        command.current_dir(parent);
    }

    #[cfg(target_os = "windows")]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command
}

#[cfg(target_os = "windows")]
fn executable_name(binary_name: &str) -> OsString {
    let mut executable_name = OsString::from(binary_name);
    executable_name.push(".exe");

    executable_name
}

#[cfg(not(target_os = "windows"))]
fn executable_name(binary_name: &str) -> OsString {
    OsString::from(binary_name)
}

fn find_file_by_name(root: &Path, file_name: &OsString) -> Option<PathBuf> {
    let mut pending = vec![root.to_path_buf()];

    while let Some(dir) = pending.pop() {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() && path.file_name() == Some(file_name.as_os_str()) {
                return Some(path);
            }

            if path.is_dir() {
                pending.push(path);
            }
        }
    }

    None
}
