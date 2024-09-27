// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data_handler;

use::tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
async fn get_file_path() -> Result<String, String> {
  let (tx, rx) = std::sync::mpsc::channel();
  FileDialogBuilder::new().add_filter("CSV or Excel Files", &["csv", "xlsx"]).pick_file(move |file_path| {
    if let Some(path) = file_path {
      tx.send(path.to_string_lossy().to_string()).expect("Failed to send file path")
    }
  });

  rx.recv().map_err(|_| "File selection cancelled".to_string())
}



fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_file_path,
      data_handler::process_excel
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
