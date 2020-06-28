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

export namespace DomTree {
  export interface Params {
    // URI of the TOML document
    uri: string;
  }

  export interface Response {
    // Syntax tree to show
    text: string;
  }

  export const METHOD = "taplo/domTree";
}

export namespace LineMappings {
  export interface Params {
    // URI of the TOML document
    uri: string;
  }

  export interface Response {
    // Line mappings
    lines: string[];
  }

  export const METHOD = "taplo/lineMappings";
}
