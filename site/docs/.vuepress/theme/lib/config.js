"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.config = void 0;
const vuepress_shared_utils_1 = require("@mr-hope/vuepress-shared-utils");
const defaultConfig_1 = require("./defaultConfig");
const resolveLocales_1 = require("./resolveLocales");
const resolveThemeConfig_1 = require("./resolveThemeConfig");
const vuepress_plugin_pwa_1 = require("@mr-hope/vuepress-plugin-pwa");
/**
 * 处理 vuepress 配置
 *
 * @param config
 */
exports.config = (config) => {
    // 合并默认配置
    vuepress_shared_utils_1.deepAssignReverse(defaultConfig_1.default, config);
    const resolvedConfig = config;
    resolveThemeConfig_1.default(resolvedConfig.themeConfig);
    resolveLocales_1.default(resolvedConfig);
    if (resolvedConfig.themeConfig.pwa)
        resolvedConfig.head = vuepress_plugin_pwa_1.head(resolvedConfig.themeConfig.pwa, config.head);
    return resolvedConfig;
};
