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
export declare class TaploLsp {
    private static lsp;
    private static initializing;
    private constructor();
    /**
     * Initialize the language server.
     *
     * After initialization, the server will be ready to accept JSON RPC messages.
     * The only way to exit is exiting the process itself.
     *
     * @param {Handlers} handlers Handlers required for the server.
     */
    static initialize(handlers: Handlers): Promise<TaploLsp>;
    /**
     * Send a JSON RPC message to the server.
     * The message must be an object, and not serialized JSON.
     */
    message(message: any): void;
}
