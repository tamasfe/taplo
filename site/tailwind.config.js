module.exports = {
  prefix: "tw-",
  content: ["./site/.vitepress/**/*.{js,ts,vue}", "./site/**/*.md"],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "#DE591B",
          light: "#E6682E",
          lighter: "#E97C49",
          dark: "#BF4D17",
          darker: "#A84314",
        },
      },
    },
  },
};
