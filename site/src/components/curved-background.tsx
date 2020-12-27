import React from "react";
import "./curved-background.scss";

export const CurvedBackground = () => (
  <div
    className="curved-background"
    style={{
      height: 0,
      pointerEvents: "none",
      zIndex: -1,
    }}
  >
    <div className="curved-background-inner"></div>
  </div>
);
