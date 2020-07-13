export const array = {
  begin: "(?<!\\w)(\\[)\\s*",
  name: "meta.array.toml",
  beginCaptures: {
    1: {
      name: "punctuation.definition.array.toml",
    },
  },
  end: "\\s*(\\])(?!\\w)",
  endCaptures: {
    1: {
      name: "punctuation.definition.array.toml",
    },
  },
  patterns: [
    {
      include: "#comment",
    },
    {
      include: "#value",
    },
  ],
};
