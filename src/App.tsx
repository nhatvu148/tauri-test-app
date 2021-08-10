import React from 'react';
import logo from './logo.svg';
import './App.css';

import { invoke } from '@tauri-apps/api/tauri'
// import { emit, listen } from '@tauri-apps/api/event'

// Invoke the command
invoke("my_custom_command");
invoke("my_custom_command2", { invokeMessage: "Hello!" });
invoke("my_custom_command3").then((message) => console.log(message));
invoke("my_custom_command4")
  .then((message) => console.log(message))
  .catch((error) => console.error(error));

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
