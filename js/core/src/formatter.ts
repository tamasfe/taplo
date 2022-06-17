/**
 * Taplo formatter options.
 */
export interface FormatterOptions {
  /**
   * Align consecutive entries vertically.
   */
  alignEntries?: boolean;
  /** Align consecutive comments after entries and items vertically.
   *
   * This applies to comments that are after entries or array items.
   */
  alignComments?: boolean;
  /**
   * Append trailing commas for multi-line arrays.
   */
  arrayTrailingComma?: boolean;
  /**
   * Expand arrays to multiple lines that exceed the maximum column width.
   */
  arrayAutoExpand?: boolean;
  /**
   * Collapse arrays that don't exceed the maximum column width and don't contain comments.
   */
  arrayAutoCollapse?: boolean;
  /**
   * Omit white space padding from single-line arrays
   */
  compactArrays?: boolean;
  /**
   * Omit white space padding from the start and end of inline tables.
   */
  compactInlineTables?: boolean;
  /**
   * Omit white space around `=`.
   */
  compactEntries?: boolean;
  /**
   * Maximum column width in characters, affects array expansion and collapse, this doesn't take whitespace into account.
   * Note that this is not set in stone, and works on a best-effort basis.
   */
  columnWidth?: number;
  /**
   * Indent based on tables and arrays of tables and their subtables, subtables out of order are not indented.
   */
  indentTables?: boolean;
  /**
   * Indent entries under tables.
   */
  indentEntries?: boolean;
  /**
   * The substring that is used for indentation, should be tabs or spaces (but technically can be anything).
   */
  indentString?: string;
  /**
   * Add trailing newline at the end of the file if not present.
   */
  trailingNewline?: boolean;
  /**
   * Alphabetically reorder keys that are not separated by empty lines.
   */
  reorderKeys?: boolean;
  /**
   * Maximum amount of allowed consecutive blank lines. This does not affect the whitespace at the end of the document, as it is always stripped.
   */
  allowedBlankLines?: number;
  /**
   * Use CRLF for line endings.
   */
  crlf?: boolean;
}
