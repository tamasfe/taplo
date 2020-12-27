import React, { useEffect, useRef, useState } from "react";
import AceEditor from "react-ace";
import { Taplo } from "@taplo/lib";
import { Button, Card, Spin } from "antd";
import {
  useEvents,
  useObservable,
  useObservableRef,
  useObservableState,
  useObservableStateRef,
} from "../util/rxjs-hooks";
import { debounce } from "rxjs/operators";
import { interval } from "rxjs";
import { Form, Switch } from "antd";

const Editor: React.FunctionComponent = () => {
  require("ace-builds/src-noconflict/mode-toml");
  require("ace-builds/src-noconflict/theme-tomorrow");

  const taplo = useRef<Taplo>();

  const [editorValue, setEditorValue] = useState("");
  const [formatOnType, setFormatOnType] = useObservableStateRef(true);

  const initTaplo = async () => {
    try {
      taplo.current = await Taplo.initialize();
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    initTaplo();
  }, []);

  const formatEditor = () => {
    try {
      if (taplo.current) {
        const formatted = taplo.current.format(editorValue, {
          options: {
            indentTables: true,
          },
        });
        setEditorValue(formatted);
      }
    } catch (e) {
      // Formatting errors.
    }
  };

  const [_, onEditorChange] = useEvents<string>(input$ => {
    input$.subscribe(s => setEditorValue(s));
    input$.pipe(debounce(() => interval(500))).subscribe(s => {
      if (!formatOnType.current || !taplo.current) {
        return;
      }

      try {
        setEditorValue(taplo.current.format(s));
      } catch (e) {
        // leave it unformatted
      }
    });
  });

  return (
    <Spin spinning={typeof taplo === "undefined"} size="large" tip="Loading...">
      <div style={{ width: "100%", height: "100%", padding: "1rem" }}>
        <div style={{ display: "flex", flexDirection: "row", width: "100%" }}>
          <Form>
            <Form.Item label="Format On Type">
              <Switch
                checked={formatOnType.current}
                onChange={setFormatOnType}
              />
            </Form.Item>
          </Form>
        </div>

        <div style={{ display: "flex", flexDirection: "row", width: "100%" }}>
          <Button onClick={formatEditor}>Format</Button>
        </div>
      </div>
      <div style={{ width: "100%", height: "100%" }}>
        <AceEditor
          placeholder="Placeholder Text"
          mode="toml"
          theme="tomorrow"
          fontSize={14}
          showPrintMargin={false}
          showGutter={true}
          highlightActiveLine={true}
          width="100%"
          height="70vh"
          value={editorValue}
          onChange={onEditorChange}
          setOptions={{
            showLineNumbers: false,
            tabSize: 2,
          }}
        />
      </div>
    </Spin>
  );
};

export default Editor;
