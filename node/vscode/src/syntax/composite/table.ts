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
  // match: "(\\{)(.*)(\\})",
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
  // captures: {
  //   1: {
  //     name: "punctuation.definition.table.inline.toml",
  //   },
  //   2: {
  //     patterns: [
  //       {
  //         include: "#comment",
  //       },
  //       {
  //         match: ",",
  //         name: "punctuation.separator.table.inline.toml",
  //       },
  //       {
  //         include: "#entryBegin",
  //       },
  //       {
  //         include: "#value",
  //       },
  //     ],
  //   },
  //   3: {
  //     name: "punctuation.definition.table.inline.toml",
  //   },
  // },
};

export const table = {
  patterns: [tableBasic, tableArray, tableInline],
};
