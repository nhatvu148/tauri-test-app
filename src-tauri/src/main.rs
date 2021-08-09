#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{command};

#[command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

fn main() {
  let msg = String::from("Hello WORLD!");
  println!("Message from Rust: {}", msg);
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
