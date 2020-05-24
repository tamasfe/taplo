// Requests that are not in the LSP spec

export interface TomlToJsonParams {
  // TOML text
  text: string;
}

export interface TomlToJsonResponse {
  // JSON text
  text?: string;
  errors?: string[];
}

export const TOML_TO_JSON = "taplo/tomlToJson";
