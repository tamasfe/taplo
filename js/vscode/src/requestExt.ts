// Requests that are not in the LSP spec

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
    kind: MessageKind;
    message: string;
  }

  export const METHOD = "taplo/messageWithOutput";
}

export namespace UpdateBuiltInSchemas {
  export interface Params {
    associations: { [key: string]: string };
  }

  export const METHOD = "taplo/updateBuiltinSchemas";
}

export namespace GetCachedSchema {
  export interface Params {
    schemaUri: string;
  }

  export interface Response {
    schemaJson?: string;
  }

  export const METHOD = "taplo/getCachedSchema";
}

export namespace CacheSchema {
  export interface Params {
    schemaUri: string;
    schemaJson: string;
  }

  export const METHOD = "taplo/cacheSchema";
}

export namespace ConfigFileChanged {
  export const METHOD = "taplo/configFileChanged";
}

export namespace WatchConfigFile {
  export interface Params {
    configPath: string;
  }

  export const METHOD = "taplo/watchConfigFile";
}