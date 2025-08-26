import type { Readable, Writable } from "node:stream";

/**
 * Environment required for several Taplo functions.
 *
 * This is required because WebAssembly is not self-contained and is sand-boxed.
 */
export interface Environment {
  /**
   * Return the current date.
   */
  now: () => Date;
  /**
   * Return the environment variable, if any.
   */
  envVar: (name: string) => string | undefined;
  /**
   * Return all environment variables as `[key, value]` pairs.
   */
  envVars: () => Array<[string, string]>;
  /**
   * Return whether the standard error output is a tty or not.
   */
  stdErrAtty: () => boolean;
  /**
   * Read `n` bytes from the standard input.
   *
   * If the returned array is empty, EOF is reached.
   *
   * This function must not return more than `n` bytes.
   */
  stdin: Readable | ((n: bigint) => Promise<Uint8Array>);
  /**
   * Write the given bytes to the standard output returning
   * the number of bytes written.
   */
  stdout: Writable | ((bytes: Uint8Array) => Promise<number>);
  /**
   * Write the given bytes to the standard error output returning
   * the number of bytes written.
   */
  stderr: Writable | ((bytes: Uint8Array) => Promise<number>);
  /**
   * Search a glob file pattern and return the matched files.
   */
  glob: (pattern: string) => Array<string>;
  /**
   * Read the contents of the file at the given path.
   */
  readFile: (path: string) => Promise<Uint8Array>;
  /**
   * Write and overwrite a file at the given path.
   */
  writeFile: (path: string, bytes: Uint8Array) => Promise<void>;
  /**
   * Turn an URL into a file path.
   */
  urlToFilePath: (url: string) => string;
  /**
   * Return whether a path is absolute.
   */
  isAbsolute: (path: string) => boolean;
  /**
   * Return whether the platform is Windows.
   */
  isWindows: () => boolean;
  /**
   * Return the path to the current working directory.
   */
  cwd: () => string;
  /**
   * Find the Taplo config file from the given directory
   * and return the path if found.
   *
   * The following files should be searched in order from the given root:
   *
   * - `.taplo.toml`
   * - `taplo.toml`
   */
  findConfigFile: (from: string) => string | undefined;
  /**
   * The fetch function if it is not defined on the global Window.
   *
   * This is required for environments like NodeJs where the fetch API is not available,
   * so a package like `node-fetch` must be used instead.
   *
   */
  fetch?: {
    fetch: any;
    Headers: any;
    Request: any;
    Response: any;
  };
}
/**
 * @private
 */
export function prepareEnv(environment: Environment) {
  if (typeof fetch === "undefined") {
    if (environment.fetch) {
      // FIXME: A lot of assumptions here...
      (global as any).Headers = environment.fetch.Headers;
      (global as any).Request = environment.fetch.Request;
      (global as any).Response = environment.fetch.Response;
      (global as any).fetch = environment.fetch.fetch;
    } else {
      console.warn(
        "fetch was not provided, HTTP operations will not be possible"
      );
    }
  }
}

/**
 * @private
 */
export function convertEnv(env: Environment): any {
  const stdin =
    typeof env.stdin === "function" ? env.stdin : streamToReadCb(env.stdin);
  const stdout =
    typeof env.stdout === "function" ? env.stdout : streamToWriteCb(env.stdout);
  const stderr =
    typeof env.stderr === "function" ? env.stderr : streamToWriteCb(env.stderr);

  return {
    js_now: env.now,
    js_env_var: env.envVar,
    js_env_vars: env.envVars,
    js_atty_stderr: env.stdErrAtty,
    js_on_stdin: stdin,
    js_on_stdout: stdout,
    js_on_stderr: stderr,
    js_glob_files: env.glob,
    js_read_file: env.readFile,
    js_write_file: env.writeFile,
    js_to_file_path: env.urlToFilePath,
    js_is_absolute: env.isAbsolute,
    js_is_windows: env.isWindows,
    js_cwd: env.cwd,
    js_find_config_file: env.findConfigFile,
  };
}

function streamToWriteCb(
  stream: Writable
): (bytes: Uint8Array) => Promise<number> {
  return (bytes) => {
    return new Promise((resolve) => {
      // FIXME: we immediately resolve as it does not matter
      //   in any of the use-cases.
      stream.write(bytes);
      resolve(bytes.length);
    });
  };
}

function streamToReadCb(stream: Readable): (n: bigint) => Promise<Uint8Array> {
  // The stream EOF event callback is immediately called after the last
  // bit of data was read, however we cannot immediately signal it as we are still returning data.
  //
  // If EOF happens, subsequent stream events will not happen, not even "end" and the promise
  // will get stuck and nodejs will terminate without any errors (found it out the hard way).
  //
  // So we keep track of EOF here and immediately return 0 bytes on the next call without
  // touching the stream.
  let eof = false;

  return (n) => {
    // Make sure that we only resolve/reject the promise once.
    // This might not be necessary, but it's better to be safe.
    let done = false;

    return new Promise((resolve, reject) => {
      if (eof) {
        return resolve(new Uint8Array());
      }

      function onReadable() {
        const data = stream.read(Number(n));
        if (data !== null) {
          if (!done) {
            done = true;
            resolve(data);
            stream.off("readable", onReadable);
          }
        }
      }

      stream.on("readable", onReadable);

      stream.once("end", () => {
        eof = true;
        if (!done) {
          done = true;
          resolve(new Uint8Array());
        }
      });

      stream.once("error", (err) => {
        if (!done) {
          done = true;
          reject(err);
        }
      });
    });
  };
}
