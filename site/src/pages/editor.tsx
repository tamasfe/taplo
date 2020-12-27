import React from "react";
import { AppHeader } from "../components/header";
import Loadable from "@loadable/component";
import { Helmet } from "react-helmet";
import Layout, { Content } from "antd/lib/layout/layout";

// @ts-ignore
const Editor = Loadable(() => import("../components/editor"));

const WebPage: React.FunctionComponent = () => {
  return (
    <div>
      <Helmet title={`Editor — Taplo`}></Helmet>
      <AppHeader path="/editor" title="Editor" />
      <Layout style={{ padding: "0 24px 24px", paddingTop: 88, minHeight: "100vh" }}>
        <Content
          style={{
            backgroundColor: "#FFF",
          }}
        >
          <Editor />
        </Content>
      </Layout>
    </div>
  );
};

export default WebPage;
