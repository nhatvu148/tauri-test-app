#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
use commands::{
  my_custom_command, my_custom_command2, my_custom_command3, my_custom_command4, read_config,
  start_server, stop_server,
};
use tauri::{Event, Manager};

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
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|app_handle, e| {
      if let Event::CloseRequested { label, api, .. } = e {
        api.prevent_close();
        let window = app_handle.get_window(&label).unwrap();
        window.emit("close-requested", ()).unwrap();
      }
    })
}
