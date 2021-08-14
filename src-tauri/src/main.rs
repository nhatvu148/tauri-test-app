#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod menu;

use commands::{
  menu_toggle, my_custom_command, my_custom_command2, my_custom_command3, my_custom_command4,
  read_config, start_server, stop_server, window_label,
};
use serde::Serialize;
use tauri::{CustomMenuItem, Event, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowUrl, api::{
    dialog,
    path::{resolve_path, BaseDirectory},
    process::{Command, CommandEvent},
  }};
use winapi::um::{
  handleapi::CloseHandle,
  processthreadsapi::{OpenProcess, TerminateProcess},
  winnt::PROCESS_TERMINATE,
};

#[derive(Serialize)]
struct Reply {
  data: String,
}

#[derive(Debug)]
pub struct AppState {
  value: u32,
  label: String,
}

fn main() {
  let msg = String::from("Hello WORLD!");
  println!("Message from Rust: {}", msg);

  let context = tauri::generate_context!();
  let script_path = resolve_path(
    context.config(),
    context.package_info(),
    "assets/index.js",
    Some(BaseDirectory::Resource),
  )
  .unwrap();

  tauri::Builder::default()
    .manage(AppState {
      value: 0,
      label: "Tauri!".into(),
    })
    .on_page_load(|window, _payload| {
      let label = window.label().to_string();
      window.listen("clicked".to_string(), move |_payload| {
        println!("got 'clicked' event on window '{}'", label);
      });

      let window_ = window.clone();
      window.listen("js-event", move |event| {
        println!("got js-event with message '{:?}'", event.payload());
        let reply = Reply {
          data: "Kyoko Murakami".to_string(),
        };
        window_
          .emit("rust-event", Some(reply))
          .expect("failed to emit");
      });

      window.listen("kill_server_process".to_string(), move |_payload| {
        let state = 
        unsafe {
          let explorer = OpenProcess(PROCESS_TERMINATE, false as i32, state.inner().value);
          TerminateProcess(explorer, 1);
          CloseHandle(explorer);
        }
        // Command::new("taskkill")
        //   .args(&["/f", "/pid", child_id.to_string().as_str()])
        //   .spawn()
        //   .expect("failed to execute process");
        println!("killed server process {}", state.inner().value);
      });
    })
    // .setup(move |app| {
    //   let window = app.get_window("main").unwrap();
    //   let script_path = script_path.to_string_lossy().to_string();
    //   tauri::async_runtime::spawn(async move {
    //     let (mut rx, _child) = Command::new("node")
    //       .args(&[script_path])
    //       .spawn()
    //       .expect("Failed to spawn node");
    //     while let Some(event) = rx.recv().await {
    //       if let CommandEvent::Stdout(line) = event {
    //         window
    //           .emit("message", Some(format!("'{}'", line)))
    //           .expect("failed to emit event");
    //       }
    //     }
    //   });
    //   Ok(())
    // })
    // .menu(menu::get_menu())
    // .on_menu_event(|event| {
    //   println!("{:?}", event.menu_item_id());
    // })
    // .system_tray(
    //   SystemTray::new().with_menu(
    //     SystemTrayMenu::new()
    //       .add_item(CustomMenuItem::new("toggle", "Toggle"))
    //       .add_item(CustomMenuItem::new("new", "New window")),
    //   ),
    // )
    // .on_system_tray_event(|app, event| match event {
    //   SystemTrayEvent::LeftClick {
    //     position: _,
    //     size: _,
    //     ..
    //   } => {
    //     let window = app.get_window("main").unwrap();
    //     window.show().unwrap();
    //     window.set_focus().unwrap();
    //   }
    //   SystemTrayEvent::MenuItemClick { id, .. } => {
    //     let item_handle = app.tray_handle().get_item(&id);
    //     match id.as_str() {
    //       "toggle" => {
    //         let window = app.get_window("main").unwrap();
    //         let new_title = if window.is_visible().unwrap() {
    //           window.hide().unwrap();
    //           "Show"
    //         } else {
    //           window.show().unwrap();
    //           "Hide"
    //         };
    //         item_handle.set_title(new_title).unwrap();
    //       }
    //       "new" => app
    //         .create_window(
    //           "new".into(),
    //           WindowUrl::App("index.html".into()),
    //           |window_builder, webview_attributes| {
    //             (window_builder.title("Tauri"), webview_attributes)
    //           },
    //         )
    //         .unwrap(),
    //       _ => {}
    //     }
    //   }
    //   _ => {}
    // })
    .invoke_handler(tauri::generate_handler![
      my_custom_command,
      my_custom_command2,
      my_custom_command3,
      my_custom_command4,
      start_server,
      stop_server,
      read_config,
      menu_toggle,
      window_label
    ])
    .build(context)
    .expect("error while building tauri application")
    .run(|app_handle, e| {
      if let Event::CloseRequested { label, api, .. } = e {
        api.prevent_close();
        let window = app_handle.get_window(&label).unwrap();
        window.emit("close-requested", ()).unwrap();
        let answer = dialog::ask("Close App", "Are you sure you want to close?");

        if let dialog::AskResponse::Yes = answer {
          stop_server().unwrap();

          window.close().unwrap();
        }
      }
    })
}
