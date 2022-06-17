import { FormatterOptions } from "./formatter";

export interface Config {
  /**
   * Files to exclude (ignore).
   *
   * A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns. Globstars (`**`) are supported.
   *
   * Relative paths are **not** relative to the configuration file, but rather depends on the tool using the configuration.
   *
   * This has priority over `include`.
   */
  exclude?: string[];
  /**
   * Formatting options.
   */
  formatting?: FormatterOptions;
  /**
   * Files to include.
   *
   * A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns. Globstars (`**`) are supported.
   *
   * Relative paths are **not** relative to the configuration file, but rather depends on the tool using the configuration.
   *
   * Omitting this property includes all files, **however an empty array will include none**.
   */
  include?: string[];
  /**
   * Rules are used to override configurations by path and keys.
   */
  rule?: Rule[];
  /**
   * Schema validation options.
   */
  schema?: SchemaOptions;
}

/**
 * A plugin to extend Taplo's capabilities.
 */
export interface Plugin {
  /**
   * Optional settings for the plugin.
   */
  settings?: {
    [k: string]: unknown;
  };
  [k: string]: unknown;
}
/**
 * A rule to override options by either name or file.
 */
export interface Rule {
  /**
   * Files that are excluded from this rule.
   *
   * A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
   *
   * Relative paths are **not** relative to the configuration file, but rather depends on the tool using the configuration.
   *
   * This has priority over `include`.
   */
  exclude?: string[];
  /**
   * Formatting options.
   */
  formatting?: FormatterOptions;
  /**
   * Files this rule is valid for.
   *
   * A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) path patterns.
   *
   * Relative paths are **not** relative to the configuration file, but rather depends on the tool using the configuration.
   *
   * Omitting this property includes all files, **however an empty array will include none**.
   */
  include?: string[];
  /**
   * Keys the rule is valid for in a document.
   *
   * A list of Unix-like [glob](https://en.wikipedia.org/wiki/Glob_(programming)) dotted key patterns.
   *
   * This allows enabling the rule for specific paths in the document.
   *
   * For example:
   *
   * - `package.metadata` will enable the rule for everything inside the `package.metadata` table, including itself.
   *
   * If omitted, the rule will always be valid for all keys.
   */
  keys?: string[];
  /**
   * The name of the rule.
   *
   * Used in `taplo::<name>` comments.
   */
  name?: string;
  /**
   * Schema validation options.
   */
  schema?: SchemaOptions;
}
/**
 * Options for schema validation and completion.
 *
 * Schemas in rules with defined keys are ignored.
 */
export interface SchemaOptions {
  /**
   * Whether the schema should be enabled or not.
   *
   * Defaults to true if omitted.
   */
  enabled?: boolean;
  /**
   * A local file path to the schema, overrides `url` if set.
   *
   * For URLs, please use `url` instead.
   */
  path?: string;
  /**
   * A full absolute Url to the schema.
   *
   * The url of the schema, supported schemes are `http`, `https`, `file` and `taplo`.
   */
  url?: string;
}
