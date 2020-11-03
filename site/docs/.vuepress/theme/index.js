"use strict";
const path_1 = require("path");
const eject_1 = require("./lib/eject");
const plugins_1 = require("./lib/plugins");
const config_1 = require("./lib/config");
const getAlias = (themeConfig, ctx) => {
    const { siteConfig } = ctx;
    // Resolve algolia
    const isAlgoliaSearch = themeConfig.algolia ||
        Object.keys((siteConfig.locales && themeConfig.locales) || {}).some((base) => themeConfig.locales[base].algolia);
    const blogEnabled = themeConfig.blog !== false;
    const commentPluginEnabled = themeConfig.comment !== false;
    const commentEnabled = themeConfig.comment &&
        themeConfig.comment.type &&
        themeConfig.comment.type !== "disable";
    const themeColorEnabled = !(themeConfig.themeColor === false && themeConfig.darkmode === "disable");
    const noopModule = "vuepress-theme-hope/lib/noopModule.js";
    return {
        "@AlgoliaSearchBox": isAlgoliaSearch
            ? path_1.resolve(__dirname, "./components/AlgoliaSearchBox.vue")
            : noopModule,
        "@BlogInfo": blogEnabled
            ? path_1.resolve(__dirname, "./components/Blog/BlogInfo.vue")
            : noopModule,
        "@BlogPage": blogEnabled
            ? path_1.resolve(__dirname, "./components/Blog/BlogPage.vue")
            : noopModule,
        "@Comment": commentPluginEnabled && commentEnabled
            ? "@mr-hope/vuepress-plugin-comment/Comment.vue"
            : noopModule,
        "@PageInfo": commentPluginEnabled
            ? "@mr-hope/vuepress-plugin-comment/PageInfo.vue"
            : noopModule,
        "@ThemeColor": themeColorEnabled
            ? path_1.resolve(__dirname, "./components/Theme/ThemeColor.vue")
            : noopModule,
    };
};
// Theme API.
const themeAPI = (themeConfig, ctx) => ({
    alias: getAlias(themeConfig, ctx),
    plugins: plugins_1.default(themeConfig),
    additionalPages: themeConfig.blog === false
        ? []
        : [
            {
                path: "/article/",
                frontmatter: { layout: "Blog" },
            },
            {
                path: "/timeline/",
                frontmatter: { layout: "Blog" },
            },
        ],
    extendCli: (cli) => {
        cli
            .command("eject-hope [targetDir]", "copy vuepress-theme-hope into .vuepress/theme for customization.")
            .option("--debug", "eject in debug mode")
            .action((dir = ".") => {
            void eject_1.default(dir);
        });
    },
});
themeAPI.config = config_1.config;
module.exports = themeAPI;
