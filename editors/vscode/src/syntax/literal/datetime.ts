const localTime = {
  name: "constant.other.time.time.toml",
  match: /\d{2}:\d{2}:\d{2}(?:\.\d+)?/.source,
};

const localDate = {
  name: "constant.other.time.date.toml",
  match: /\d{4}\-\d{2}\-\d{2}/.source,
};

const localDateTime = {
  captures: {
    1: {
      name: "constant.other.time.datetime.local.toml",
    },
  },
  match: "(\\d{4}\\-\\d{2}\\-\\d{2}T\\d{2}:\\d{2}:\\d{2}(?:\\.\\d+)?)",
};

const offsetDateTime = {
  captures: {
    1: {
      name: "constant.other.time.datetime.offset.toml",
    },
  },
  match:
    "(?<!\\w)(\\d{4}\\-\\d{2}\\-\\d{2}[T| ]\\d{2}:\\d{2}:\\d{2}(?:\\.\\d+)?(?:Z|[\\+\\-]\\d{2}:\\d{2}))(?!\\w)",
};

// ordered, must be reversed
export const datetime = [
  offsetDateTime,
  localDateTime,
  localDate,
  localTime,
];
