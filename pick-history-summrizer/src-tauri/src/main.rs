// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn get_file_path(file_path: String) -> Result<String, String> {
  // 実際のパスの取得は、環境に応じたロジックを実装
  if std::path::Path::new(&file_path).exists() {
    Ok(file_path)
  } else {
    Err("ファイルが見つかりません".into())
  }
  
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_file_path
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
