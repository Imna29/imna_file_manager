use serde::Serialize;
use std::fs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_directory_content, open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DirectoryItem {
    // Consider using owned String instead of references
    name: String,
    path: String,
    is_dir: bool,
    parent: Option<String>,
}

#[tauri::command]
fn get_directory_content(directory: String) -> Vec<DirectoryItem> {
    let entries = fs::read_dir(directory);

    let mut items = Vec::new();

    if let Ok(entries) = entries {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let is_dir = path.is_dir();
            let parent = path.parent().map(|p| p.to_str().unwrap().to_string());
            items.push(DirectoryItem {
                name,
                is_dir,
                parent,
                path: path.to_str().unwrap().to_string(),
            });
        }
    }

    items
}

#[tauri::command]
fn open_file(path: String) {
    open::that(path).unwrap();
}
