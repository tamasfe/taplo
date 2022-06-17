export * from "./environment";
export * from "./formatter";
export * as Lsp from "./lsp";
export * from "./config";

/**
 * Byte range within a TOML document.
 */
export interface Range {
  /**
   * Start byte index.
   */
  start: number;
  /**
   * Exclusive end index.
   */
  end: number;
}

