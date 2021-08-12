#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod utils;
use commands::{
  my_custom_command, my_custom_command2, my_custom_command3, my_custom_command4, read_config,
  start_server, stop_server,
};

fn main() {
  let msg = String::from("Hello WORLD!");
  println!("Message from Rust: {}", msg);
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      my_custom_command,
      my_custom_command2,
      my_custom_command3,
      my_custom_command4,
      start_server,
      stop_server,
      read_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
