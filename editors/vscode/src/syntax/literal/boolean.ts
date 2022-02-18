export const boolean = [
  {
    match: "(?<!\\w)(true|false)(?!\\w)",
    captures: {
      1: {
        name: "constant.language.boolean.toml",
      },
    },
  },
];


