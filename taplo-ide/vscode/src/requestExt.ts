// Requests that are not in the LSP spec

import { Uri } from "vscode";

export namespace TomlToJson {
  export interface Params {
    // TOML text
    text: string;
  }

  export interface Response {
    // JSON text
    text?: string;
    errors?: string[];
  }

  export const METHOD = "taplo/tomlToJson";
}

export namespace SyntaxTree {
  export interface Params {
    // URI of the TOML document
    uri: string;
  }

  export interface Response {
    // Syntax tree to show
    text: string;
  }

  export const METHOD = "taplo/syntaxTree";
}


export namespace MessageWithOutput {
  export const enum MessageKind {
    Info = "info",
    Warn = "warn",
    Error = "error",
  }

  export interface Params {
    kind: MessageKind,
    message: string;
  }

  export const METHOD = "taplo/messageWithOutput";
}
