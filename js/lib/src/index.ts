import {
  Config,
  convertEnv,
  Environment,
  FormatterOptions,
  prepareEnv,
} from "@taplo/core";
import loadTaplo from "../../../crates/taplo-wasm/Cargo.toml";
import { objectCamel } from "./util";

/**
 * Options for the format function.
 */
export interface FormatOptions {
  /**
   * Options to pass to the formatter.
   */
  options?: FormatterOptions;

  /**
   * Taplo configuration, this can be parsed
   * from files like `taplo.toml` or provided manually.
   */
  config?: Config;
}

/**
 * Options for TOML Lint.
 */
export interface LintOptions {
  /**
   * Taplo configuration, this can be parsed
   * from `.taplo.toml` or provided manually.
   */
  config?: Config;
}
/**
 * An lint error.
 */
export interface LintError {
  /**
   * A range within the TOML document if any.
   */
  range?: Range;
  /**
   * The error message.
   */
  error: string;
}

/**
 * The object returned from the lint function.
 */
export interface LintResult {
  /**
   * Lint errors, if any.
   *
   * This includes syntax, semantic and schema errors as well.
   */
  errors: Array<LintError>;
}

/**
 * This class allows for usage of the library in a synchronous context
 * after being asynchronously initialized once.
 *
 * It cannot be constructed with `new`, and instead must be
 * created by calling `initialize`.
 *
 * Example usage:
 *
 * ```js
 * import { Taplo } from "taplo";
 *
 * // Somewhere at the start of your app.
 * const taplo = await Taplo.initialize();
 * // ...
 * // The other methods will not return promises.
 * const formatted = taplo.format(tomlDocument);
 * ```
 */
export class Taplo {
  private static taplo: any | undefined;
  private static initializing: boolean = false;

  private constructor(private env: Environment) {
    if (!Taplo.initializing) {
      throw new Error(
        `an instance of Taplo can only be created by calling the "initialize" static method`
      );
    }
  }

  public static async initialize(env?: Environment): Promise<Taplo> {
    if (typeof Taplo.taplo === "undefined") {
      Taplo.taplo = await loadTaplo();
    }
    Taplo.taplo.initialize();

    const environment = env ?? browserEnvironment();
    prepareEnv(environment);

    Taplo.initializing = true;
    const t = new Taplo(environment);
    Taplo.initializing = false;

    return t;
  }

  /**
   * Lint a TOML document, this function returns
   * both syntax and semantic (e.g. conflicting keys) errors.
   *
   * If a JSON schema is found in the config, the TOML document will be validated with it
   * only if it is syntactically valid.
   *
   * Example usage:
   *
   * ```js
   * const lintResult = await taplo.lint(tomlDocument, {
   *   config: { schema: { url: "https://example.com/my-schema.json" } },
   * });
   *
   * if (lintResult.errors.length > 0) {
   *   throw new Error("the document is invalid");
   * }
   * ```
   *
   * @param toml TOML document.
   * @param options Optional additional options.
   */
  public async lint(toml: string, options?: LintOptions): Promise<LintResult> {
    return await Taplo.taplo.lint(
      convertEnv(this.env),
      toml,
      objectCamel(options?.config ?? {})
    );
  }

  /**
   * Format the given TOML document.
   *
   * @param toml TOML document.
   * @param options Optional format options.
   */
  public format(toml: string, options?: FormatOptions): string {
    try {
      return Taplo.taplo.format(
        convertEnv(this.env),
        toml,
        options?.options ?? {},
        objectCamel(options?.config ?? {})
      );
    } catch (e) {
      throw new Error(e);
    }
  }

  /**
   * Encode the given JavaScript object to TOML.
   *
   * @throws If the given object cannot be serialized to TOML.
   *
   * @param data JSON compatible JavaScript object or JSON string.
   */
  public encode(data: object | string): string {
    if (typeof data !== "string") {
      data = JSON.stringify(data);
    }

    try {
      return Taplo.taplo.from_json(data);
    } catch (e) {
      throw new Error(e);
    }
  }

  /**
   * Decode the given TOML string to a JavaScript object.
   *
   * @throws If data is not valid TOML.
   *
   * @param {string} data TOML string.
   */
  public decode<T extends object = any>(data: string): T;

  /**
   * Convert the given TOML string to JSON.
   *
   * @throws If data is not valid TOML.
   *
   * @param data TOML string.
   * @param {boolean} parse Whether to keep the JSON in a string format.
   */
  public decode(data: string, parse: false): string;

  public decode<T extends object = any>(
    data: string,
    parse: boolean = true
  ): T | string {
    let v: string;
    try {
      v = Taplo.taplo.to_json(data);
    } catch (e) {
      throw new Error(e);
    }

    if (parse) {
      return JSON.parse(v);
    } else {
      return v;
    }
  }
}

/**
 * A very limited default environment inside a browser.
 */
function browserEnvironment(): Environment {
  return {
    cwd: () => "",
    envVar: () => "",
    envVars: () => [["", ""]],
    findConfigFile: () => undefined,
    glob: () => [],
    isAbsolute: () => true,
    now: () => new Date(),
    readFile: () => Promise.reject("not implemented"),
    writeFile: () => Promise.reject("not implemented"),
    stderr: async bytes => {
      console.error(new TextDecoder().decode(bytes));
      return bytes.length;
    },
    stdErrAtty: () => false,
    stdin: () => Promise.reject("not implemented"),
    stdout: async bytes => {
      console.log(new TextDecoder().decode(bytes));
      return bytes.length;
    },
    urlToFilePath: (url: string) => url.slice("file://".length),
  };
}
