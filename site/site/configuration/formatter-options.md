# Formatter Options

This page contains a list of formatting options the formatter accepts.

::: warning

In some environments (e.g. in Visual Studio Code and JavaScript) the option keys are _camelCase_ to better fit the conventions. For example `align_entries` becomes `alignEntries`.

In some environments (e.g., Visual Studio Code), one needs to reload the extension to let the settings take effect.

:::

|        option         |                                                          description                                                           | default value  |
| :-------------------: | :----------------------------------------------------------------------------------------------------------------------------: | :------------: |
|     align_entries     |       Align entries vertically. Entries that have table headers, comments, or blank lines between them are not aligned.        |     false      |
|    align_comments     | Align consecutive comments after entries and items vertically. This applies to comments that are after entries or array items. |      true      |
| array_trailing_comma  |                                           Put trailing commas for multiline arrays.                                            |      true      |
|   array_auto_expand   |                   Automatically expand arrays to multiple lines when they exceed `column_width` characters.                    |      true      |
|  array_auto_collapse  |                                     Automatically collapse arrays if they fit in one line.                                     |      true      |
|    compact_arrays     |                                       Omit whitespace padding inside single-line arrays.                                       |      true      |
| compact_inline_tables |                                         Omit whitespace padding inside inline tables.                                          |     false      |
|  inline_table_expand  |                                       Expand values (e.g. arrays) inside inline tables.                                        |      true      |
|    compact_entries    |                                                  Omit whitespace around `=`.                                                   |     false      |
|     column_width      |                          Target maximum column width after which arrays are expanded into new lines.                           |       80       |
|     indent_tables     |                                            Indent subtables if they come in order.                                             |     false      |
|    indent_entries     |                                                  Indent entries under tables.                                                  |     false      |
|     indent_string     |                        Indentation to use, should be tabs or spaces but technically could be anything.                         | 2 spaces (" ") |
|   trailing_newline    |                                              Add trailing newline to the source.                                               |      true      |
|     reorder_keys      |                               Alphabetically reorder keys that are not separated by blank lines.                               |     false      |
|    reorder_arrays     |                           Alphabetically reorder array values that are not separated by blank lines.                           |     false      |
| reorder_inline_tables |                                             Alphabetically reorder inline tables.                                              |     false      |
|  allowed_blank_lines  |                                     The maximum amount of consecutive blank lines allowed.                                     |       2        |
|         crlf          |                                                     Use CRLF line endings.                                                     |     false      |
