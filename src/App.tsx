import React, { useState, useEffect, useRef } from "react";

import { getName } from "@tauri-apps/api/app";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { invoke } from "@tauri-apps/api/tauri";
import { readDir, readTextFile } from "@tauri-apps/api/fs";
import { relaunch, exit } from "@tauri-apps/api/process";
import { open } from "@tauri-apps/api/shell";
import {
  appWindow,
  WindowManager,
  WebviewWindow,
} from "@tauri-apps/api/window";
import { dataDir } from "@tauri-apps/api/path";
import {
  sendNotification,
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/api/notification";
import { emit, listen } from "@tauri-apps/api/event";

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
import ReactBSAlert from "react-bootstrap-sweetalert";
import NotificationAlert from "react-notification-alert";

// appWindow.listen("tauri://close-requested", async () => {
// if (await window.confirm("Are you sure you want to close?")) {
// await appWindow.close();
// }
// });
let label: string;
appWindow.listen("tauri://window-created", function (event: any) {
  label = event.payload.label as string;
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
  const [tempClientPort, setTempClientPort] = useState(0);
  const [serverPort, setServerPort] = useState(0);
  const [isPortChanged, setIsPortChanged] = useState(false);
  const [isServerOn, setIsServerOn] = useState(false);
  const [alert, setAlert] = useState(null);
  const notificationAlertRef = useRef(null);

  const notify = (type: string, message: string) => {
    let options = {
      place: "tc",
      message: (
        <div className="alert-text">
          <span className="alert-title" data-notify="title">
            Warning
          </span>
          <span data-notify="message">{message}</span>
        </div>
      ),
      type: type,
      icon: "ni ni-bell-55",
      autoDismiss: 7,
    };
    // @ts-ignore
    notificationAlertRef.current.notificationAlert(options);
  };

  useEffect(() => {
    (async () => {
      const confPath = await invoke("read_config");
      console.log(confPath);
      const content = await readTextFile(confPath as string);
      const content_arr = content.split("\r\n");
      const portClient = content_arr
        .filter((cf) => cf.startsWith("PORT_CLIENT"))[0]
        .split("=")[1];
      const portProd = content_arr
        .filter((cf) => cf.startsWith("PORT_PROD"))[0]
        .split("=")[1];

      setClientPort(Number(portClient));
      setTempClientPort(Number(portClient));
      setServerPort(Number(portProd));
    })();
  }, []);

  useEffect(() => {
    (async () => {
      // const unlisten = await listen("rust-event", (message: any) => {
      //   console.log(message.payload.data);
      //   // setLog(message.payload.data);
      //   // @ts-ignore
      //   window.term.echo(message.payload.data);
      // })

      const unlisten2 = await listen("message", (event) => {
        // @ts-ignore
        window.term.echo(event.payload);
      });

      return () => {
        // if (unlisten) {
        //   unlisten();
        // }
        if (unlisten2) {
          unlisten2();
        }
      };
    })();
  }, []);

  const warningMessage = (message: string) => {
    setAlert(
      // @ts-ignore
      <ReactBSAlert
        warning
        style={{}}
        title={message}
        onConfirm={() => {
          setAlert(null);
        }}
        onCancel={() => {}}
        confirmBtnCssClass="btn-secondary"
        cancelBtnBsStyle="danger"
        confirmBtnText="OK"
        // cancelBtnText="OK"
        // showCancel
        btnSize=""
      />
    );
  };

  return (
    <div className="main-content">
      {alert}
      <div className="rna-wrapper">
        <NotificationAlert ref={notificationAlertRef} />
      </div>
      <Container className="mt-5" fluid>
        <Card>
          <CardBody>
            <Row align="center">
              <div
                className="col-6"
                style={{
                  display: "flex",
                  justifyContent: "center",
                  alignItems: "center",
                }}
              >
                <Form style={{ marginTop: 21 }}>
                  <FormGroup className="row">
                    <Label
                      className="form-control-label"
                      htmlFor="example-number-input"
                      md="5"
                    >
                      Port Number
                    </Label>
                    <Col md="7">
                      <Input
                        value={tempClientPort}
                        onChange={(event) => {
                          setTempClientPort(Number(event.target.value));
                          setIsPortChanged(true);
                        }}
                        id="example-number-input"
                        type="number"
                      />
                    </Col>
                  </FormGroup>
                </Form>
              </div>
              <div
                className="col-6"
                style={{
                  display: "flex",
                  justifyContent: "center",
                  alignItems: "center",
                }}
              >
                {isPortChanged && isServerOn ? (
                  <Button
                    color="success"
                    type="button"
                    onClick={() => {
                      // @ts-ignore
                      window.term.exec("clear", true);
                      
                      invoke("start_server", {
                        port: tempClientPort,
                        portProd: serverPort,
                      });
                      setClientPort(tempClientPort);
                      setIsPortChanged(false);
                    }}
                  >
                    Restart Server
                  </Button>
                ) : !isServerOn ? (
                  <Button
                    color="success"
                    type="button"
                    onClick={() => {
                      setIsServerOn(true);
                      setIsPortChanged(false);
                      let port = clientPort;
                      if (clientPort !== tempClientPort) {
                        port = tempClientPort;
                        setClientPort(port);
                      }
                      invoke("start_server", {
                        port: port,
                        portProd: serverPort,
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
                      // emit("kill_server_process");

                      invoke("stop_server");
                      // @ts-ignore
                      window.term.exec("clear", true);
                      setClientPort(tempClientPort);
                      setIsServerOn(false);
                    }}
                  >
                    Stop Server
                  </Button>
                )}
                <Button
                  color="info"
                  type="button"
                  onClick={async () => {
                    if (!isServerOn) {
                      if (!(await isPermissionGranted())) {
                        const permit = await requestPermission();
                        console.log(permit);
                      }
                      sendNotification({
                        title: "JMU-DT Web Controller",
                        body: "Server is not running!",
                        icon: "",
                      });

                      // emit('clicked', 'message from ' + label);

                      // invoke("window_label", { text: "Hello" });

                      // @ts-ignore
                      // window.term.echo("clicked");
                      // notify("warning", "Server is not running!");
                      // warningMessage("Server is not running!");
                    } else {
                      await open(`http://localhost:${clientPort}`);
                      // emit("js-event", "this is the payload string");
                    }
                  }}
                >
                  Launch Browser
                </Button>
              </div>
              {/* <div className="col-3" style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
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
                    console.log(await dataDir());

                    // const files = await readDir("./");
                    // console.log(files);

                    setCounter(0);

                    setMessage("");
                  }}
                >
                  Clear
                </Button>
              </div> */}
            </Row>
            <Row align="center">
              <Col style={{ marginTop: 20 }}>
                <h2>
                  {!isServerOn
                    ? "Server stopped!"
                    : `JMU-DT Web Server is running on port ${clientPort}!`}
                </h2>
                <h2>
                  {message} {counter ? `${counter}!` : ""}
                </h2>
              </Col>
            </Row>
          </CardBody>
        </Card>
      </Container>
    </div>
  );
};

export default App;
