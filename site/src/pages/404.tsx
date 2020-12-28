import { Button } from "antd";
import Layout, { Content } from "antd/lib/layout/layout";
import * as React from "react";
import { Helmet } from "react-helmet";
import { AppHeader } from "../components/header";

const NotFoundPage = () => {
  return (
    <>
      <Helmet title={`Not Found — Taplo`}></Helmet>
      <AppHeader path="/404"></AppHeader>
      <Content
        style={{
          paddingTop: 64,
        }}
      >
        <Layout
          style={{
            backgroundColor: "#FFF",
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            minHeight: "50vh",
            justifyContent: "center",
            marginBottom: "2rem",
          }}
        >
          <div
            style={{
              fontSize: "10rem",
              fontWeight: "bold",
              opacity: 0.1,
              pointerEvents: "none",
            }}
          >
            404
          </div>
          <div
            style={{
              fontSize: "2rem",
              marginBottom: "1rem",
            }}
          >
            The page was not found.
          </div>
          <Button type="primary">
            <a href="/">To Home Page</a>
          </Button>
        </Layout>
      </Content>
    </>
  );
};

export default NotFoundPage;
