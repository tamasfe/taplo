import React, { useEffect, useRef, useState } from "react";
import AceEditor from "react-ace";
import { FormatterOptions, Taplo } from "@taplo/lib";
import { Button, Card, Divider, Space, Spin, Tooltip } from "antd";
import {
  useEvents,
  useObservableState,
  useObservableStateRef,
} from "../util/rxjs-hooks";
import { debounce, filter } from "rxjs/operators";
import { interval } from "rxjs";
import { Form, Switch } from "antd";

import CopyIcon from "mdi-react/ContentCopyIcon";
import JsonIcon from "mdi-react/CodeJsonIcon";

const Editor: React.FunctionComponent = () => {
  require("ace-builds/src-noconflict/mode-toml");
  require("ace-builds/src-noconflict/theme-tomorrow");

  const taplo = useRef<Taplo>();
  const [taploInitialized, setTaploInitialized] = useState(false);

  const currentDate = () => {
    let d = new Date().toISOString().replace("T", " ").replace("Z", "");

    let periodIdx = d.indexOf(".");

    if (periodIdx !== -1) {
      d = d.slice(0, periodIdx);
    }

    return d;
  };

  const formatOpts: FormatterOptions = {
    arrayAutoCollapse: false,
    arrayAutoExpand: false,
  };

  const [convertJson, setConvertJson] = useObservableStateRef(true);

  const [formatOnType, setFormatOnType] = useObservableStateRef(false);

  const [formatTooltip, setFormatTooltip] = useState("");
  const [
    formatTooltipVisible,
    setFormatTooltipVisible,
  ] = useObservableState<boolean>(false, e$ => {
    e$.pipe(debounce(() => interval(1500))).subscribe(v => {
      if (v) {
        setFormatTooltipVisible(false);
      }
    });
  });

  const [copyTooltip, setCopyTooltip] = useState("");
  const [
    copyTooltipVisible,
    setCopyTooltipVisible,
  ] = useObservableState<boolean>(false, e$ => {
    e$.pipe(debounce(() => interval(1500))).subscribe(v => {
      if (v) {
        setCopyTooltipVisible(false);
      }
    });
  });

  const [copyJsonTooltip, setCopyJsonTooltip] = useState("");
  const [
    copyJsonTooltipVisible,
    setCopyJsonTooltipVisible,
  ] = useObservableState<boolean>(false, e$ => {
    e$.pipe(debounce(() => interval(1500))).subscribe(v => {
      if (v) {
        setCopyJsonTooltipVisible(false);
      }
    });
  });

  const [editorValue, setEditorValue] = useState(`[example]
current_date_utc = ${currentDate()}
message = "edit me!"
`);

  const initTaplo = async () => {
    try {
      taplo.current = await Taplo.initialize();
      setTaploInitialized(true);
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
          options: formatOpts,
        });
        setEditorValue(formatted);
        setFormatTooltip("Formatted!");
        setFormatTooltipVisible(true);
      }
    } catch (e) {
      console.error(e);
      setFormatTooltip("Formatting failed!");
      setFormatTooltipVisible(true);
    }
  };

  const copy = async () => {
    if (editorValue.trim().length === 0) {
      setCopyTooltip("There is nothing to copy!");
      setCopyTooltipVisible(true);
      return;
    }

    try {
      await navigator.clipboard.writeText(editorValue);
      setCopyTooltip("Copied!");
      setCopyTooltipVisible(true);
    } catch (e) {
      console.error(e);
      setCopyTooltip("Copying Failed!");
      setCopyTooltipVisible(true);
    }
  };

  const copyJson = async () => {
    if (!taplo.current) {
      return;
    }

    if (editorValue.trim().length === 0) {
      setCopyJsonTooltip("There is nothing to copy!");
      setCopyJsonTooltipVisible(true);
      return;
    }

    try {
      await navigator.clipboard.writeText(
        taplo.current.decode(editorValue, false)
      );
      setCopyJsonTooltip("JSON Copied!");
      setCopyJsonTooltipVisible(true);
    } catch (e) {
      console.error(e);
      setCopyJsonTooltip("Copying Failed!");
      setCopyJsonTooltipVisible(true);
    }
  };

  const [_, onEditorChange] = useEvents<string>(input$ => {
    input$.subscribe(s => setEditorValue(s));
    input$.pipe(debounce(() => interval(500))).subscribe(s => {
      if (!formatOnType.current || !taplo.current) {
        return;
      }

      try {
        setEditorValue(taplo.current.format(s, { options: formatOpts }));
      } catch (e) {
        // leave it unformatted
      }
    });
  });

  return (
    <Spin spinning={!taploInitialized} size="large" tip="Loading...">
      <div
        style={{
          width: "100%",
          height: "100%",
          padding: "1rem",
          paddingBottom: 0,
        }}
      >
        <div style={{ display: "flex", flexDirection: "row", width: "100%" }}>
          <Form style={{ display: "flex", flexDirection: "row" }}>
            <Space>
              <Form.Item label="Format On Type">
                <Switch
                  checked={formatOnType.current}
                  onChange={setFormatOnType}
                />
              </Form.Item>

              <Tooltip title="Automatically convert JSON on paste">
                <Form.Item label="Convert JSON">
                  <Switch
                    checked={convertJson.current}
                    onChange={setConvertJson}
                  />
                </Form.Item>
              </Tooltip>
            </Space>
          </Form>
        </div>

        <div style={{ display: "flex", flexDirection: "row", width: "100%" }}>
          <Space>
            <Tooltip title={formatTooltip} visible={formatTooltipVisible}>
              <Button onClick={formatEditor}>Format</Button>
            </Tooltip>
            <Tooltip title={copyTooltip} visible={copyTooltipVisible}>
              <Button onClick={copy}>
                <div style={{ display: "flex", alignItems: "center" }}>
                  <CopyIcon
                    size="1rem"
                    style={{ marginRight: "0.5rem", marginLeft: "-0.5rem" }}
                  />
                  Copy
                </div>
              </Button>
            </Tooltip>
            <Tooltip title={copyJsonTooltip} visible={copyJsonTooltipVisible}>
              <Button onClick={copyJson}>
                <div style={{ display: "flex", alignItems: "center" }}>
                  <JsonIcon
                    size="1rem"
                    style={{ marginRight: "0.5rem", marginLeft: "-0.5rem" }}
                  />
                  Copy JSON
                </div>
              </Button>
            </Tooltip>
          </Space>
        </div>
      </div>
      <Divider type="horizontal" />
      <div
        style={{
          width: "calc(100% - 2rem)",
          height: "70vh",
          margin: "1rem",
          marginTop: 0,
          border: "1px solid rgba(0,0,0,0.1)",
          boxShadow: "0px 0px 0.5rem 0.1rem rgba(0,0,0, 0.05)",
        }}
      >
        <AceEditor
          mode="toml"
          theme="tomorrow"
          fontSize={14}
          showPrintMargin={false}
          showGutter={false}
          highlightActiveLine={true}
          width="100%"
          height="100%"
          value={editorValue}
          onChange={onEditorChange}
          onLoad={editor => {
            (editor as any).onPaste = (val: string) => {
              if (!convertJson.current || !taplo.current) {
                editor.session.insert(editor.getCursorPosition(), val);
                return;
              }
              try {
                editor.session.insert(
                  editor.getCursorPosition(),
                  taplo.current.encode(val)
                );
              } catch (e) {
                console.log(e);
                editor.session.insert(editor.getCursorPosition(), val);
              }
            };
          }}
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
