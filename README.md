# Tauri First App

## Normal Setup

- yarn add -D @tauri-apps/cli
- yarn tauri init
- yarn tauri info
- yarn tauri dev
- yarn tauri build --debug
- yarn tauri deps install
- yarn tauri deps update

## Tauri Init

- https://geekjr.github.io/reactTauri.html

## Steps to install Tauri with React:

- cargo install tauri-bundler
- npx create-react-app app-name
- yarn add tauri (obsolete, use tauri-apps/cli)
- yarn add -D @tauri-apps/cli
- Add scripts to package.json:

```json
"tauri": "tauri",
"dev": "yarn tauri dev",
"bundle": "yarn tauri build",
```

- yarn tauri init
- yarn build
- yarn bundle
