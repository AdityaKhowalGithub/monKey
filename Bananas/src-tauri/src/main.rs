// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::Manager;

#[derive(Serialize, Deserialize)]
struct FileInfo {
    name: String,
    size: String,
    icon: String,
}

#[derive(Serialize, Deserialize)]
struct DirectoryInfo {
    name: String,
    icon: String,
    children: Vec<String>,
}

#[tauri::command]
fn list_files(dir: &str) -> Result<Vec<FileInfo>, String> {
    let mut files = Vec::new();
    let path = PathBuf::from(dir);

    if path.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_type = entry.file_type().map_err(|e| e.to_string())?;
            let file_name = entry.file_name().into_string().map_err(|e| e.to_string())?;
            let metadata = entry.metadata().map_err(|e| e.to_string())?;

            let size = if file_type.is_file() {
                format!("{} MB", metadata.len() / 1024 / 1024)
            } else {
                "".to_string()
            };

            let icon = if file_type.is_file() {
                "mdi:file-document-outline".to_string()
            } else {
                "mdi:folder".to_string()
            };

            files.push(FileInfo {
                name: file_name,
                size,
                icon,
            });
        }
    } else {
        return Err("Not a directory".to_string());
    }

    Ok(files)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
