const tableBasic = {
  name: "meta.table.toml",
  match: "^\\s*(\\[)([^\\[\\]]*)(\\])",
  captures: {
    1: {
      name: "punctuation.definition.table.toml",
    },
    2: {
      patterns: [
        {
          match: "[^\\s.]+",
          name: "variable.key.table.toml",
        },
        {
          match: "\\.",
          name: "punctuation.separator.dot",
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
  match: "^\\s*(\\[\\[)([^\\[\\]]*)(\\]\\])",
  captures: {
    1: {
      name: "punctuation.definition.array.table.toml",
    },
    2: {
      patterns: [
        {
          match: "[^\\s.]+",
          name: "variable.key.array.table.toml",
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
  begin: "(?<!\\w)(\\{)\\s*",
  name: "meta.table.inline.toml",
  beginCaptures: {
    1: {
      name: "punctuation.definition.table.inline.toml",
    },
  },
  end: "\\s*(\\})(?!\\w)",
  endCaptures: {
    1: {
      name: "punctuation.definition.table.inline.toml",
    },
  },
  patterns: [
    {
      include: "#entry",
    },
    {
      include: "#literal",
    },
  ],
};

export const table = {
  patterns: [tableBasic, tableArray, tableInline],
};
