"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const vuepress_shared_utils_1 = require("@mr-hope/vuepress-shared-utils");
const resolveEncrypt_1 = require("./resolveEncrypt");
const { checkLang, getLocale, lang2path, path2lang } = vuepress_shared_utils_1.i18n;
/**
 * 处理主题配置
 *
 * @param themeConfig
 * @param baseLang
 */
const setThemeLocales = (themeConfig, baseLang) => {
    /** 默认语言对应的路径 */
    const baseLangPath = lang2path(baseLang);
    // 设置根目录语言配置
    themeConfig.locales["/"] = Object.assign(Object.assign(Object.assign({}, getLocale(baseLang)), (themeConfig.locales[baseLangPath] || {})), (themeConfig.locales["/"] || {}));
    // 处理其他语言
    Object.keys(themeConfig.locales).forEach((path) => {
        if (path === "/")
            return;
        const lang = path2lang(path);
        themeConfig.locales[path] = Object.assign(Object.assign({}, getLocale(lang)), themeConfig.locales[path]);
    });
};
/**
 * 处理主题配置
 *
 * @param themeConfig 主题配置
 */
const resolveThemeConfig = (themeConfig) => {
    /** 主目录对应语言 */
    const { baseLang = "en-US" } = themeConfig;
    // 如果主目录启用了未适配的语言，抛出错误
    if (!checkLang(baseLang))
        throw new Error("Base lang not supported. Make a PR to https://github.com/Mister-Hope/vuepress-theme-hope/blob/master/packages/shared-utils/lib/i18n/config.ts first!");
    setThemeLocales(themeConfig, baseLang);
    if (themeConfig.encrypt)
        resolveEncrypt_1.default(themeConfig.encrypt);
};
exports.default = resolveThemeConfig;
