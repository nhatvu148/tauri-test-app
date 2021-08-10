import React, { useState } from 'react';
import './App.css';

import { getName } from '@tauri-apps/api/app';
import { readText, writeText } from '@tauri-apps/api/clipboard';
import { invoke } from '@tauri-apps/api/tauri';
import { readDir, readTextFile } from '@tauri-apps/api/fs';
import { relaunch } from '@tauri-apps/api/process';
import { open } from '@tauri-apps/api/shell';
import { appWindow } from '@tauri-apps/api/window';

appWindow.listen('tauri://move', ({ event, payload }) => {
  // @ts-ignore
  const { x, y } = payload // payload here is a `PhysicalPosition`
  console.log(x, y);
})
// import { emit, listen } from '@tauri-apps/api/event'

// Invoke the command
invoke("my_custom_command");
invoke("my_custom_command4")
  .then((message) => console.log(message))
  .catch((error) => console.error(error));

const App = () => {
  const [message, setMessage] = useState("");

  return (
    <div className="App">
      <h1>Hello World!</h1>
      <button onClick={() => {
        invoke("my_custom_command2", { invokeMessage: "Hello!" });
        invoke("my_custom_command3").then((message) => {
          setMessage(message as string);
          console.log(message)
        });
      }}>Click me</button>
      <button onClick={async () => {
        const name = await getName();
        console.log(name);
        writeText("Kyoko");
        const text = await readText();
        console.log(text);

        const files = await readDir("./");
        console.log(files);
        const content = await readTextFile("./Cargo.toml");
        console.log(content);

        setMessage("");
      }}>Clear</button>
      <button onClick={() => {
        invoke("run_cmd");;
      }}>Run Command Line</button>
      <button onClick={() => { relaunch(); }}>Relaunch</button>
      <button onClick={async () => { await open('https://chiyoda.e-technostar.com'); }}>Open Something</button>
      <h2>{message}</h2>
    </div >
  );
}

export default App;
