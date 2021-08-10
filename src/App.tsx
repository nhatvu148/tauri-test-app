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
  const [counter, setCounter] = useState(0);
  const [clientPort, setClientPort] = useState(0);
  const [serverPort, setServerPort] = useState(0);

  return (
    <div className="App">
      <h1>JMU-DT Web Controller</h1>
      <button onClick={() => {
        invoke("my_custom_command2", { invokeMessage: "Hello!" });
        invoke("my_custom_command3").then((message) => {
          setMessage(message as string);
          setCounter(prev => prev + 1);
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

        setCounter(0);

        setMessage("");
      }}>Clear</button>
      {serverPort === 0 ?
        <button onClick={() => {
          const newServerPort = 4000;
          const newCientPort = 50505;
          setServerPort(newServerPort);
          setClientPort(newCientPort);

          invoke("start_server", { port: newCientPort, portProd: newServerPort });;
        }}>Start Server</button>
        :
        <button onClick={() => {
          const newServerPort = 0;
          const newCientPort = 0;
          setServerPort(newServerPort);
          setClientPort(newCientPort);

          invoke("stop_server");;
        }}>Stop Server</button>
      }

      <button onClick={() => { relaunch(); }}>Relaunch</button>
      <button onClick={async () => { await open(`http://localhost:${clientPort}`); }}>Launch JMU-DT Web</button>

      <h2>{serverPort === 0 ? "Server stopped!" : `JMU-DT Web Server is running on port ${clientPort}!`}</h2>
      <h2>{message} {counter ? `${counter}!` : ""}</h2>
    </div >
  );
}

export default App;
