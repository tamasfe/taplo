"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const vuepress_shared_utils_1 = require("@mr-hope/vuepress-shared-utils");
const { path2lang } = vuepress_shared_utils_1.i18n;
/**
 * 生成对应语言配置
 *
 * @param {object} config vuepress配置
 */
const resolveLocales = (config) => {
    // 确保存在 locales
    if (!config.locales)
        config.locales = {};
    /** 主目录对应语言 */
    const { baseLang = "en-US" } = config.themeConfig;
    const { locales } = config;
    // 设置根目录语言配置
    locales["/"] = Object.assign({ lang: baseLang }, (locales["/"] || {}));
    // 处理其他语言
    Object.keys(config.themeConfig.locales).forEach((path) => {
        if (path === "/")
            return;
        locales[path] = Object.assign({ lang: path2lang(path) }, (locales[path] || {}));
    });
};
exports.default = resolveLocales;
