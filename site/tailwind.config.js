module.exports = {
  prefix: "tw-",
  content: ["./site/.vitepress/**/*.{js,ts,vue}", "./site/**/*.md"],
  plugins: [require("daisyui")],
  daisyui: {
    base: false,
    darkTheme: "dark",
    themes: [
      {
        taplo: {
          primary: "#DE591B",
          secondary: "#1BA0DE",
          accent: "#1BDEBB",
        },
      },
    ],
  },
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
