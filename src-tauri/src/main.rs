#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
  fs::{self, File},
  io::{prelude::*, LineWriter},
  process::Command,
  thread, time,
  time::{Duration, Instant},
};
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

#[command]
fn start_server(port: u16, port_prod: u16) -> Result<String, String> {
  restart_nginx(port, port_prod);

  Command::new("taskkill")
    .args(&["/fi", "WINDOWTITLE eq JMU-DT Web Server*"])
    .spawn()
    .expect("failed to execute process");

  let duration = time::Duration::from_millis(100);
  thread::sleep(duration);

  Command::new("cmd")
    .args(&[
      "/C",
      "cd",
      "C:/Users/nhatv/Work/TechnoStar/jmu-dt",
      "&&",
      "start",
      "start_server.cmd",
      port_prod.to_string().as_str(),
    ])
    .spawn()
    .expect("failed to execute process");

  Ok("Command line worked!".into())
}

#[command]
fn stop_server() -> Result<String, String> {
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
      stop_server
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn restart_nginx(port: u16, port_prod: u16) -> std::io::Result<()> {
  let nginx_dir =
    String::from("C:/Users/nhatv/Work/TechnoStar/jmu-dt/nwbuilder/package.nw/bin/nginx");
  let nginx_dir_text = format!("{}/conf/nginx.conf", nginx_dir);

  let ok = edit_nginx(port, port_prod, &nginx_dir_text);

  Command::new("taskkill")
    .args(&["/F", "/im", "nginx_dt.exe"])
    .spawn()
    .expect("failed to execute process");

  let duration = time::Duration::from_millis(100);
  thread::sleep(duration);

  Command::new("cmd")
    .args(&[
      "/C",
      "cd",
      nginx_dir.as_str(),
      "&&",
      "start",
      "nginx_dt.exe",
    ])
    .spawn()
    .expect("failed to execute process");

  Ok(())
}

fn edit_nginx(port: u16, port_prod: u16, conf_file: &String) -> std::io::Result<()> {
  let port_prod_text = format!("        server localhost:{};", port_prod);
  let port_text = format!("        listen       {};", port);

  let lines = vec![
    "",
    "#user  nobody;",
    "worker_processes  1;",
    "",
    "#error_log  logs/error.log;",
    "#error_log  logs/error.log  notice;",
    "#error_log  logs/error.log  info;",
    "",
    "#pid        logs/nginx.pid;",
    "",
    "",
    "events {",
    "    worker_connections  1024;",
    "}",
    "",
    "",
    "http {",
    "    include       mime.types;",
    "    default_type  application/octet-stream;",
    "",
    "    #log_format  main  '$remote_addr - $remote_user [$time_local] \"$request\" '",
    "    #                  '$status $body_bytes_sent \"$http_referer\" '",
    "    #                  '\"$http_user_agent\" \"$http_x_forwarded_for\"';",
    "",
    "    #access_log  logs/access.log  main;",
    "",
    "    sendfile        on;",
    "    #tcp_nopush     on;",
    "",
    "    #keepalive_timeout  0;",
    "    keepalive_timeout  65;",
    "",
    "    #gzip  on;",
    "",
    "    upstream backend-server {",
    port_prod_text.as_str(),
    "    }",
    "",
    "    server {",
    port_text.as_str(),
    "        server_name  localhost;",
    "",
    "        location / {",
    "            root   ../../client/build;",
    "            index  index.html;",
    "",
    "            try_files $uri /index.html;",
    "        }",
    "",
    "        location /api/ {",
    "            proxy_pass http://backend-server;",
    "        }",
    "",
    "        error_page  404              /404.html;",
    "",
    "        # redirect server error pages to the static page /50x.html",
    "        #",
    "        error_page   500 502 503 504  /50x.html;",
    "        location = /50x.html {",
    "            root   html;",
    "        }",
    "    }",
    "",
    "    server {",
    "        listen       5005;",
    "        server_name  localhost;",
    "",
    "        location / {",
    "            root ../../cug_viewer/dist/example-cug-viewer;",
    "		    index index.html;",
    "",
    "            try_files $uri /index.html;",
    "        }",
    "    }",
    "}",
  ];

  let file = File::create(conf_file)?;
  let mut file = LineWriter::new(file);

  for sub_string in &lines {
    file.write_all(sub_string.as_bytes())?;
    file.write_all(b"\n")?;
  }

  file.flush()?;

  Ok(())
}
