export const entryBegin = {
  name: "meta.entry.toml",
  match: "\\s*((?<!\\s*=\\s*)[^\\[{,=#]*)\\s*(=)",
  captures: {
    1: {
      patterns: [
        {
          match: "[^\\s.]+",
          name: "variable.key.toml",
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
