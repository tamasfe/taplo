"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
/** 默认配置 */
exports.default = {
    base: process.env.VuePress_BASE || "/",
    temp: "./node_modules/.temp",
    theme: "hope",
    themeConfig: { locales: {} },
    /** 是否只支持常青树浏览器 */
    evergreen: true,
};
