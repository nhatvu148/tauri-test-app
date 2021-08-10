import React, { useState } from "react";

import { getName } from "@tauri-apps/api/app";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { invoke } from "@tauri-apps/api/tauri";
import { readDir, readTextFile } from "@tauri-apps/api/fs";
import { relaunch, exit } from "@tauri-apps/api/process";
import { open } from "@tauri-apps/api/shell";
import { appWindow, WindowManager } from "@tauri-apps/api/window";
// import { emit, listen } from '@tauri-apps/api/event'

import {
  Button,
  Card,
  CardHeader,
  CardBody,
  Label,
  FormGroup,
  Form,
  Input,
  Container,
  Row,
  Col,
} from "reactstrap";

appWindow.listen("tauri://move", ({ event, payload }) => {
  // @ts-ignore
  const { x, y } = payload; // payload here is a `PhysicalPosition`
  console.log(x, y);
});
appWindow.listen("tauri://close-requested", () => {
  alert("Are you sure you want to close?");
});

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
    <div className="main-content">
      <Container className="mt-5" fluid>
        <Card>
          <CardBody>
            <Row>
              <Col md="8">
                <Form>
                  <FormGroup className="row">
                    <Label
                      className="form-control-label"
                      htmlFor="example-number-input"
                      md="4"
                    >
                      Port Number
                    </Label>
                    <Col md="6">
                      <Input
                        defaultValue="23"
                        id="example-number-input"
                        type="number"
                      />
                    </Col>
                  </FormGroup>
                </Form>
              </Col>
              <Col md="4">
                {serverPort === 0 ? (
                  <Button
                    color="success"
                    type="button"
                    onClick={() => {
                      const newServerPort = 4000;
                      const newCientPort = 50505;
                      setServerPort(newServerPort);
                      setClientPort(newCientPort);

                      invoke("start_server", {
                        port: newCientPort,
                        portProd: newServerPort,
                      });
                    }}
                  >
                    Start Server
                  </Button>
                ) : (
                  <Button
                    color="danger"
                    type="button"
                    onClick={() => {
                      const newServerPort = 0;
                      const newCientPort = 0;
                      setServerPort(newServerPort);
                      setClientPort(newCientPort);

                      invoke("stop_server");
                    }}
                  >
                    Stop Server
                  </Button>
                )}

                <Button
                  color="info"
                  type="button"
                  onClick={async () => {
                    if (serverPort === 0) {
                      alert("Server is not running!");
                    } else {
                      await open(`http://localhost:${clientPort}`);
                    }
                  }}
                >
                  Launch JMU-DT Web
                </Button>
              </Col>
            </Row>

            <Button
              color="success"
              type="button"
              onClick={() => {
                invoke("my_custom_command2", { invokeMessage: "Hello!" });
                invoke("my_custom_command3").then((message) => {
                  setMessage(message as string);
                  setCounter((prev) => prev + 1);
                  console.log(message);
                });
              }}
            >
              Click me
            </Button>
            <Button
              color="success"
              type="button"
              onClick={async () => {
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
              }}
            >
              Clear
            </Button>

            <h2>
              {serverPort === 0
                ? "Server stopped!"
                : `JMU-DT Web Server is running on port ${clientPort}!`}
            </h2>
            <h2>
              {message} {counter ? `${counter}!` : ""}
            </h2>
          </CardBody>
        </Card>
      </Container>
    </div>
  );
};

export default App;
