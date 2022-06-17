export const entryBegin = {
  name: "meta.entry.toml",
  match: `\\s*((?:(?:(?:[A-Za-z0-9_+-]+)|(?:"[^"]+")|(?:'[^']+'))\\s*\\.?\\s*)+)\\s*(=)`,
  captures: {
    1: {
      patterns: [
        {
          match: `(?:[A-Za-z0-9_+-]+)|(?:"[^"]+")|(?:'[^']+')`,
          name: "support.type.property-name.toml",
        },
        {
          match: "\\.",
          name: "punctuation.separator.dot.toml",
        },
      ],
    },
    2: {
      name: "punctuation.eq.toml",
    },
  },
};
