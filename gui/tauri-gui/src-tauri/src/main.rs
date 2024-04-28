// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tauri::command]
fn get_version() -> String {
  return String::from(VERSION);
}

#[tauri::command]
fn uses_system_z3() -> bool {
  #[cfg(feature = "bundle_z3")]
  return false;

  #[cfg(not(feature = "bundle_z3"))]
  return true;
}


fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_version,uses_system_z3])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

