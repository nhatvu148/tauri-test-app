#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod menu;

use commands::{
  menu_toggle, my_custom_command, my_custom_command2, my_custom_command3, my_custom_command4,
  read_config, start_server, stop_server,
};
use serde::Serialize;
use tauri::{
  api::dialog, CustomMenuItem, Event, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
  WindowBuilder, WindowUrl,
};

#[derive(Serialize)]
struct Reply {
  data: String,
}

fn main() {
  let msg = String::from("Hello WORLD!");
  println!("Message from Rust: {}", msg);
  tauri::Builder::default()
    // .on_page_load(|window, _| {
    //   let window_ = window.clone();
    //   window.listen("js-event", move |event| {
    //     println!("got js-event with message '{:?}'", event.payload());
    //     let reply = Reply {
    //       data: "something else".to_string(),
    //     };
    //     window_
    //       .emit("rust-event", Some(reply))
    //       .expect("failed to emit");
    //   });
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
      menu_toggle
    ])
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|app_handle, e| {
      if let Event::CloseRequested { label, api, .. } = e {
        api.prevent_close();
        let window = app_handle.get_window(&label).unwrap();
        window.emit("close-requested", ()).unwrap();
        let answer = dialog::ask("Close App", "Are you sure you want to close?");

        if let dialog::AskResponse::Yes = answer {
          window.close().unwrap();
        }
      }
    })
}
