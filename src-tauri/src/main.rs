#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::command;

#[command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

#[command]
fn my_custom_command2(invoke_message: String) {
  println!(
    "I was invoked from JS, with this message: {}",
    invoke_message
  );
}

#[command]
fn my_custom_command3() -> String {
  "Hello from Rust!".into()
}

#[command]
fn my_custom_command4() -> Result<String, String> {
  if true {
    Err("This failed!".into())
  } else {
    Ok("This worked!".into())
  }
}

fn main() {
  let msg = String::from("Hello WORLD!");
  println!("Message from Rust: {}", msg);
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      my_custom_command,
      my_custom_command2,
      my_custom_command3,
      my_custom_command4
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
