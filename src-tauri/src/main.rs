// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{net::{TcpListener, TcpStream}, fmt::format};

use tauri::utils::config;




fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![sendNewPage, first_time_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn sendNewPage(page_num:&str) -> String {
  let parsed_number: Result<i32, _> = page_num.parse();

    match parsed_number {
        Ok(number) => {
            return format!("Pager Nummer: {} wurde gepingt", number);
        }
        Err(e) => {
            return format!("Failed to parse number: {}", e);
        }
    }
}


#[tauri::command]
fn first_time_file() -> String {
//config file
  let config_path = tauri::api::path::config_dir();
  format!("Path: {:?}", config_path.unwrap())
}
