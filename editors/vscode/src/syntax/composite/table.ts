const tableBasic = {
  name: "meta.table.toml",
  match: `^\\s*(\\[)\\s*((?:(?:(?:[A-Za-z0-9_+-]+)|(?:"[^"]+")|(?:'[^']+'))\\s*\\.?\\s*)+)\\s*(\\])`,
  captures: {
    1: {
      name: "punctuation.definition.table.toml",
    },
    2: {
      patterns: [
        {
          match: `(?:[A-Za-z0-9_+-]+)|(?:"[^"]+")|(?:'[^']+')`,
          name: "support.type.property-name.table.toml",
        },
        {
          match: "\\.",
          name: "punctuation.separator.dot.toml",
        },
      ],
    },
    3: {
      name: "punctuation.definition.table.toml",
    },
  },
};

const tableArray = {
  name: "meta.array.table.toml",
  match:  `^\\s*(\\[\\[)\\s*((?:(?:(?:[A-Za-z0-9_+-]+)|(?:"[^"]+")|(?:'[^']+'))\\s*\\.?\\s*)+)\\s*(\\]\\])`,
  captures: {
    1: {
      name: "punctuation.definition.array.table.toml",
    },
    2: {
      patterns: [
        {
          match: `(?:[A-Za-z0-9_+-]+)|(?:"[^"]+")|(?:'[^']+')`,
          name: "support.type.property-name.array.toml",
        },
        {
          match: "\\.",
          name: "punctuation.separator.dot.toml",
        },
      ],
    },
    3: {
      name: "punctuation.definition.array.table.toml",
    },
  },
};

export const tableInline = {
  begin: "(\\{)",
  end: "(\\})",
  name: "meta.table.inline.toml",
  beginCaptures: {
    1: {
      name: "punctuation.definition.table.inline.toml",
    },
  },
  endCaptures: {
    1: {
      name: "punctuation.definition.table.inline.toml",
    },
  },
  patterns: [
    {
      include: "#comment",
    },
    {
      match: ",",
      name: "punctuation.separator.table.inline.toml",
    },
    {
      include: "#entryBegin",
    },
    {
      include: "#value",
    },
  ],
};

export const table = {
  patterns: [tableBasic, tableArray, tableInline],
};
