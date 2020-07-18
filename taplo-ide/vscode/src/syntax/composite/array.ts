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
    // Needed because inline tables can effectively
    // be split across multiple lines, but their
    // rule isn't prepared for it
    // {
    //   match: "(\\s*[\\{\\,]\\s*.*=)",
    //   name: "asd.toml",
    //   captures: {
    //     1: {
    //       patterns: [
    //         {
    //           include: "#entryBegin",
    //         },
    //       ],
    //     },
    //   },
    // },
    {
      include: "#value",
    },
  ],
};
