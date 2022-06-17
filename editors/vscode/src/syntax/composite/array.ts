export const array = {
  name: "meta.array.toml",
  begin: "(?<!\\w)(\\[)\\s*",
  end: "\\s*(\\])(?!\\w)",
  beginCaptures: {
    1: {
      name: "punctuation.definition.array.toml",
    },
  },
  endCaptures: {
    1: {
      name: "punctuation.definition.array.toml",
    },
  },
  patterns: [
    {
      match: ",",
      name: "punctuation.separator.array.toml",
    },
    {
      include: "#comment",
    },
    {
      include: "#value",
    },
  ],
};
