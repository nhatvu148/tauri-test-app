#[cfg(windows)]
extern crate winapi;
use directories::BaseDirs;
use std::{
  path::{Path, PathBuf},
  process::Command,
  thread, time,
};
use tauri::{
  api::process::{Command as TauriCommand, CommandEvent},
  command, State, Window,
};
pub mod utils;
use utils::restart_nginx;

use crate::AppState;

#[command]
pub fn my_custom_command() {
  println!("I was invoked from JS!");
}

#[command]
pub fn my_custom_command2(invoke_message: String) {
  println!(
    "I was invoked from JS, with this message: {}",
    invoke_message
  );
}

#[command]
pub fn my_custom_command3() -> String {
  "Hello from Rust!".into()
}

#[command]
pub fn my_custom_command4() -> Result<String, String> {
  if true {
    Err("This failed!".into())
  } else {
    Ok("This worked!".into())
  }
}

#[command]
pub fn start_server(
  window: Window,
  state: State<'static, AppState>,
  port: u16,
  port_prod: u16,
) -> Result<String, String> {
  restart_nginx(port, port_prod).unwrap();

  Command::new("taskkill")
    .args(&["/fi", "WINDOWTITLE eq JMU-DT Web Server*"])
    .spawn()
    .expect("failed to execute process");

  let duration = time::Duration::from_millis(100);
  thread::sleep(duration);

  tauri::async_runtime::spawn(async move {
    let path = PathBuf::from(r"C:\Users\nhatv\Work\TechnoStar\jmu-dt");
    let (mut rx, child) = TauriCommand::new("cmd")
      .args(&["/C", "start_server.cmd", port_prod.to_string().as_str()])
      .current_dir(path)
      .spawn()
      .expect("Failed to spawn cmd");

    let child_id = child.pid();

    state.inner().value = child_id;
    println!("child_id: {}", state.inner().value);

    while let Some(event) = rx.recv().await {
      if let CommandEvent::Stdout(line) = event {
        window
          .emit("message", Some(format!("{}", line)))
          .expect("failed to emit event");
      } else if let CommandEvent::Stderr(line) = event {
        window
          .emit("message", Some(format!("{}", line)))
          .expect("failed to emit event");
      }
    }
  });

  // Command::new("cmd")
  //   .args(&[
  //     "/C",
  //     "cd",
  //     "C:/Users/nhatv/Work/TechnoStar/jmu-dt",
  //     "&&",
  //     "start",
  //     "start_server.cmd",
  //     port_prod.to_string().as_str(),
  //   ])
  //   .spawn()
  //   .expect("failed to execute process");

  Ok("Command line worked!".into())
}

#[command]
pub fn read_config() -> Result<String, String> {
  if let Some(base_dirs) = BaseDirs::new() {
    let mut new_path = base_dirs.data_local_dir().to_string_lossy().to_string();
    new_path.push_str("\\jmudt-web-server\\settings\\.env");
    println!("{}", new_path); //jmudt-web-server\settings

    let my_path = Path::new(&new_path);
    dotenv::from_path(&my_path).ok();

    let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set");
    let db_database = std::env::var("DB_DATABASE").expect("DB_DATABASE must be set");
    let db_user = std::env::var("DB_USER").expect("DB_USER must be set");
    let db_pass = std::env::var("DB_PASS").expect("DB_PASS must be set");
    let port_client = std::env::var("PORT_CLIENT").expect("PORT_CLIENT must be set");
    let port_prod = std::env::var("PORT_PROD").expect("PORT_PROD must be set");
    println!(
      "{} {} {} {} {} {}",
      db_host, db_database, db_user, db_pass, port_client, port_prod
    );

    Ok(new_path.into())
  } else {
    Err("This failed!".into())
  }
}

#[command]
pub fn stop_server() -> Result<String, String> {
  Command::new("taskkill")
    .args(&["/F", "/im", "nginx_dt.exe"])
    .spawn()
    .expect("failed to execute process");
  Command::new("taskkill")
    .args(&["/F", "/im", "nginx_dt.exe"])
    .spawn()
    .expect("failed to execute process");

  Command::new("taskkill")
    .args(&["/fi", "WINDOWTITLE eq JMU-DT Web Server*"])
    .spawn()
    .expect("failed to execute process");
  // TODO: pass Err to Front end

  Ok("Command line worked!".into())
}

#[command]
pub async fn menu_toggle(window: Window) {
  window.menu_handle().toggle().unwrap();
}

#[command]
pub fn window_label(window: Window, text: String) {
  println!("window label: {} and {}", window.label(), text);
}
