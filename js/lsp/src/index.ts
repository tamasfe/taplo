// Wrapper over the WASM module.
//
// Proxies all messages between the IPC
// channel and the WASM module.
//
// And provides some utilities.

// @ts-ignore
import loadTaplo from "../../../taplo-lsp/Cargo.toml";

/**
 * The language server relies on these methods in order
 * to run in NodeJS.
 */
export interface Handlers {
  /**
   * Whether the environment is Windows.
   * Required for case-sensitivity.
   */
  isWindows: () => boolean;
  /**
   * Handle a JSON RPC message from the server.
   * The message is an object, and not serialized JSON.
   */
  sendMessage: (message: any) => void;
  /**
   * Read a file.
   */
  readFile: (path: string) => Promise<Uint8Array>;
  /**
   * Write a file.
   */
  writeFile: (path: string, data: Uint8Array) => Promise<void>;
  /**
   * Whether the given path is an absolute filesystem path or not.
   */
  isAbsolutePath: (path: string) => boolean;
  /**
   * Whether a file or directory exists at the given path.
   */
  fileExists: (path: string) => boolean;
  /**
   * Make a directory path recursively. (`mkdir -p` in linux)
   */
  mkdir: (path: string) => void;
  /**
   * Whether the file at the given path is older than the given timestamp.
   * The timestamp is in UNIX milliseconds.
   */
  needsUpdate: (path: string, newTimestamp: number) => boolean;
}

export class TaploLsp {
  private static lsp: any | undefined;
  private static initializing: boolean = false;

  private constructor() {
    if (!TaploLsp.initializing) {
      throw new Error(
        `an instance of TaploLsp can only be created by calling the "initialize" static method`
      );
    }
  }

  /**
   * Initialize the language server.
   * 
   * After initialization, the server will be ready to accept JSON RPC messages.
   * The only way to exit is exiting the process itself.
   * 
   * @param {Handlers} handlers Handlers required for the server.
   */
  public static async initialize(handlers: Handlers) {
    if (typeof TaploLsp.lsp === "undefined") {
      (global as any).isWindows = handlers.isWindows;
      (global as any).sendMessage = handlers.sendMessage;
      (global as any).readFile = handlers.readFile;
      (global as any).writeFile = handlers.writeFile;
      (global as any).isAbsolutePath = handlers.isAbsolutePath;
      (global as any).fileExists = handlers.fileExists;
      (global as any).mkdir = handlers.mkdir;
      (global as any).needsUpdate = handlers.needsUpdate;

      TaploLsp.lsp = await loadTaplo();
      TaploLsp.lsp.initialize();
    }

    TaploLsp.initializing = true;
    const t = new TaploLsp();
    TaploLsp.initializing = false;

    return t;
  }

  /**
   * Send a JSON RPC message to the server.
   * The message must be an object, and not serialized JSON.
   */
  public message(message: any) {
    TaploLsp.lsp.message(message);
  }
}
