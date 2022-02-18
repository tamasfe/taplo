const escape = [
  {
    match: '\\\\([btnfr"\\\\\\n/ ]|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})',
    name: "constant.character.escape.toml",
  },
  {
    match: '\\\\[^btnfr/"\\\\\\n]',
    name: "invalid.illegal.escape.toml",
  },
];

const stringSingle = {
  name: "string.quoted.single.basic.line.toml",
  begin: '"',
  end: '"',
  patterns: escape,
};

const stringBlock = {
  name: "string.quoted.triple.basic.block.toml",
  begin: '"""',
  end: '"""',
  patterns: escape,
};

// do not need escape characters
const literalStringSingle = {
  name: "string.quoted.single.literal.line.toml",
  begin: "'",
  end: "'",
};

const literalStringBlock = {
  name: "string.quoted.triple.literal.block.toml",
  begin: "'''",
  end: "'''",
};

// ordered, block must be before single
export const string = [
  stringBlock,
  stringSingle,
  literalStringBlock,
  literalStringSingle,
];
