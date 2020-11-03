const { description } = require("../../package");
const { config } = require("vuepress-theme-hope");

module.exports = config({
  /**
   * Ref：https://v1.vuepress.vuejs.org/config/#title
   */
  title: "Taplo",
  /**
   * Ref：https://v1.vuepress.vuejs.org/config/#description
   */
  description: `A versatile, feature-rich TOML toolkit.`,

  /**
   * Extra tags to be injected to the page HTML `<head>`
   *
   * ref：https://v1.vuepress.vuejs.org/config/#head
   */
  head: [
    ["meta", { name: "theme-color", content: "#3eaf7c" }],
    ["meta", { name: "apple-mobile-web-app-capable", content: "yes" }],
    [
      "meta",
      { name: "apple-mobile-web-app-status-bar-style", content: "black" },
    ],
  ],

  /**
   * Theme configuration, here is the default theme configuration for VuePress.
   *
   * ref：https://v1.vuepress.vuejs.org/theme/default-theme-config.html
   */
  themeConfig: {
    blog: false,
    repo: "",
    editLinks: false,
    docsDir: "",
    editLinkText: "",
    lastUpdated: true,
    themeColor: false,
    fullscreen: false,
    logo: "/taplo-icon.svg",
    darkmode: "auto-switch",
    typescript: true,
    smoothScroll: true,
    iconPrefix: "mdi-",
    pageInfo: [],
    copyCode: false,
    nav: [
      {
        text: "CLI",
        link: "/cli/",
      },
      {
        text: "Library",
        link: "/lib/",
      },
      {
        text: "Configuration",
        link: "/configuration/",
      },
      {
        text: "Repository",
        link: "https://github.com/tamasfe/taplo",
        icon: "github"
      },
    ],
    sidebarDepth: 4,
    sidebar: "auto",
  },

  /**
   * Apply plugins，ref：https://v1.vuepress.vuejs.org/zh/plugin/
   */
  plugins: [
    "@vuepress/plugin-back-to-top",
    "@vuepress/plugin-medium-zoom",
    "@goy/svg-icons",
  ],
});
