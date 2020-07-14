export const entry = {
  name: "meta.entry.toml",
  patterns: [
    // {
    //   name: "invalid.illegal.key.missing.toml",
    //   match: "(\\s*=.*)$",
    // },
    // {
    //   name: "invalid.illegal.value.missing.toml",
    //   match: "(\\s*[A-Za-z_\\-][A-Za-z0-9_\\-]*\\s*=)(?=\\s*$)",
    // },
    {
      begin: "\\s*([^\\[{,]*)\\s*(=)\\s*",
      beginCaptures: {
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
      end: "($|(?==)|\\,|\\s*(?=\\}))",
      patterns: [
        {
          include: "#comment",
        },
        {
          include: "#value",
        },
        // {
        //   include: "#illegal",
        // },
      ],
    },
  ],
};
