use std::{
  fs::File,
  io::{prelude::*, LineWriter},
  process::Command,
  thread, time,
};

pub fn restart_nginx(port: u16, port_prod: u16) -> std::io::Result<()> {
  let nginx_dir =
    String::from("C:/Users/nhatv/Work/TechnoStar/jmu-dt/nwbuilder/package.nw/bin/nginx");
  let nginx_dir_text = format!("{}/conf/nginx.conf", nginx_dir);

  let _ok = edit_nginx(port, port_prod, &nginx_dir_text);

  Command::new("taskkill")
    .args(&["/F", "/im", "nginx_dt.exe"])
    .spawn()
    .expect("failed to execute process");
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
