import Layout, { Content } from "antd/lib/layout/layout";
import * as React from "react";
import { Helmet } from "react-helmet";
import { CurvedBackground } from "../components/curved-background";
import { AppHeader } from "../components/header";
import "./index.scss";
import taploIcon from "../assets/taplo-icon.svg";
import { Button, Card, Space } from "antd";
import OpenInNewIcon from "mdi-react/OpenInNewIcon";
import CodeIcon from "mdi-react/XmlIcon";

const IndexPage = () => {
  return (
    <div>
      <Helmet title={`Taplo — A TOML Toolkit`}></Helmet>
      <AppHeader path="/" hideLogo></AppHeader>
      <Content
        style={{
          paddingTop: 64,
        }}
      >
        <Layout
          style={{
            backgroundColor: "#FFF",
            minHeight: "20vh",
            alignItems: "center",
            justifyContent: "center",
            marginBottom: "2rem",
          }}
        >
          <div
            style={{
              display: "flex",
              marginTop: "3rem",
              marginBottom: "3rem",
              flexDirection: "row",
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <img src={taploIcon} style={{ height: "10rem" }}></img>

            <div
              style={{
                display: "flex",
                flexDirection: "column",
                marginLeft: "1rem",
              }}
            >
              <h1 style={{ fontSize: "4rem", margin: "unset" }}>Taplo</h1>
              <h2 style={{ fontSize: "1.6rem", margin: "unset" }}>
                A versatile, feature-rich TOML toolkit.
              </h2>
            </div>
          </div>

          <div
            style={{
              display: "flex",
              flexDirection: "row",
              flexWrap: "wrap",
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <Button
              href="/editor"
              type="primary"
              size="large"
              color="blue-5"
              style={{ margin: "0.5rem" }}
            >
              <div style={{ display: "flex", alignItems: "center" }}>
                <CodeIcon
                  style={{ marginRight: "0.5rem", marginLeft: "-0.5rem" }}
                />
                Try it!
              </div>
            </Button>
            <Button
              href="https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml"
              type="primary"
              target="_blank"
              size="large"
              style={{ margin: "0.5rem" }}
            >
              <div style={{ display: "flex", alignItems: "center" }}>
                <OpenInNewIcon
                  style={{ marginRight: "0.5rem", marginLeft: "-0.5rem" }}
                />
                Visual Studio Code
              </div>
            </Button>
            <Button
              href="/cli"
              type="primary"
              size="large"
              style={{ margin: "0.5rem" }}
            >
              Command Line Tool
            </Button>
            <Button
              href="/lib"
              type="primary"
              size="large"
              style={{ margin: "0.5rem" }}
            >
              Library
            </Button>
          </div>
        </Layout>
        <CurvedBackground />
        <div
          className="main-bg"
          style={{
            marginTop: "5rem",
            minHeight: "60vh",
            width: "100%",
            display: "flex",
            flexDirection: "column",
            flexWrap: "wrap",
          }}
        >
          <div className="card-row">
            <Card
              title="Latest TOML Version"
              extra={<a href="https://toml.io/en/v1.0.0">v1.0.0</a>}
              hoverable
              style={{ width: 300, height: 160 }}
            >
              <h4 style={{ textAlign: "center" }}>
                Support for the latest TOML specification
              </h4>
            </Card>
            <Card
              title="Validation"
              hoverable
              style={{ width: 300, height: 170 }}
            >
              <h4 style={{ textAlign: "center" }}>
                Syntax validation of TOML documents
              </h4>
            </Card>
            <Card
              title="Formatting"
              extra={<a href="/configuration#formatting-options">more</a>}
              hoverable
              style={{ width: 300, height: 170 }}
            >
              <h4 style={{ textAlign: "center" }}>Configurable formatting</h4>
            </Card>
          </div>
          <div className="card-row">
            <Card
              title="JSON Schema"
              hoverable
              extra={<a href="/configuration#schemas">more</a>}
              style={{ width: 300, height: 170 }}
            >
              <h4 style={{ textAlign: "center" }}>
                Completion and validation based on JSON Schema
              </h4>
            </Card>
            <Card
              title="Cross-Platform"
              hoverable
              style={{ width: 300, height: 170 }}
            >
              <h4 style={{ textAlign: "center" }}>
                Available wherever Rust can compile
              </h4>
            </Card>
          </div>
        </div>
      </Content>
    </div>
  );
};

export default IndexPage;
