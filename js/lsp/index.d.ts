/**
 * Additional methods that are not in the official LSP specification.
 */
export declare namespace Methods {
    /**
     * Sent from the client to the server.
     *
     * Convert a TOML text to JSON.
     */
    namespace TomlToJson {
        interface Params {
            /**
             * TOML text
             */
            text: string;
        }
        interface Response {
            /**
             * JSON text
             */
            text?: string;
            errors?: string[];
        }
        const METHOD = "taplo/tomlToJson";
    }
    /**
     * Sent from the client to the server.
     *
     * Convert a JSON text to TOML.
     */
    namespace JsonToToml {
        interface Params {
            /**
             * JSON text
             */
            text: string;
        }
        interface Response {
            /**
             * TOML text
             */
            text?: string;
            error?: string;
        }
        const METHOD = "taplo/jsonToToml";
    }
    /**
     * Sent from the client to the server.
     *
     * Print the syntax tree for a document for debugging.
     */
    namespace SyntaxTree {
        interface Params {
            /**
             * URI of the TOML document,
             * it must have been opened.
             */
            uri: string;
        }
        interface Response {
            /**
             * The syntax tree.
             */
            text: string;
        }
        const METHOD = "taplo/syntaxTree";
    }
    /**
     * Sent from the server to the client.
     *
     * Used for showing a message to the user with
     * a button that navigates to the server's logs.
     */
    namespace MessageWithOutput {
        const enum MessageKind {
            Info = "info",
            Warn = "warn",
            Error = "error"
        }
        interface Params {
            kind: MessageKind;
            message: string;
        }
        const METHOD = "taplo/messageWithOutput";
    }
    /**
     * Sent from the client to the server.
     *
     * Set the path the server should use for caching,
     * this is optional.
     */
    namespace CachePath {
        interface Params {
            path: string;
        }
        const METHOD = "taplo/cachePath";
    }
}
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
