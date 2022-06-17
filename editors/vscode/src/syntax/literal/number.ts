const integer = {
  match: "(?<!\\w)((?:[\\+\\-]?(0|([1-9](([0-9]|_[0-9])+)?))))(?!\\w)",
  captures: {
    1: {
      name: "constant.numeric.integer.toml",
    },
  },
};

const float = {
  match:
    "(?<!\\w)([\\+\\-]?(0|([1-9](([0-9]|_[0-9])+)?))(?:(?:\\.([0-9]+))?[eE][\\+\\-]?[1-9]_?[0-9]*|(?:\\.[0-9_]*)))(?!\\w)",
  captures: {
    1: {
      name: "constant.numeric.float.toml",
    },
  },
};

const special = [
  {
    match: "(?<!\\w)([\\+\\-]?inf)(?!\\w)",
    captures: {
      1: {
        name: "constant.numeric.inf.toml",
      },
    },
  },
  {
    match: "(?<!\\w)([\\+\\-]?nan)(?!\\w)",
    captures: {
      1: {
        name: "constant.numeric.nan.toml",
      },
    },
  },
];

const leadingZero = [
  {
    match:
      "(?<!\\w)((?:0x(([0-9a-fA-F](([0-9a-fA-F]|_[0-9a-fA-F])+)?))))(?!\\w)",
    captures: {
      1: {
        name: "constant.numeric.hex.toml",
      },
    },
  },
  {
    match: /(?<!\w)(0o[0-7](_?[0-7])*)(?!\w)/.source,
    captures: {
      1: {
        name: "constant.numeric.oct.toml",
      },
    },
  },
  {
    match: /(?<!\w)(0b[01](_?[01])*)(?!\w)/.source,
    captures: {
      1: {
        name: "constant.numeric.bin.toml",
      },
    },
  },
];

// ordered, float before integer
export const number = [float, integer].concat(special, leadingZero);
