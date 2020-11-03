import { Route } from "vue-router";
/** 锚点匹配正则 */
export declare const hashRE: RegExp;
/** 后缀匹配正则 */
export declare const extRE: RegExp;
/** `/` 结尾匹配正则 */
export declare const endingSlashRE: RegExp;
/** 外部链接匹配正则 */
export declare const outboundRE: RegExp;
/**
 * 去除路径的文件后缀与锚点
 *
 * @param path 需要处理的路径
 */
export declare const normalize: (path: string) => string;
/**
 * 获取路径中的锚点
 *
 * @param path 待处理的路径
 */
export declare const getHash: (path: string) => string | void;
/**
 * 判断路径是否是外部链接
 *
 * @param path 待判断的路径
 */
export declare const isExternal: (path: string) => boolean;
/**
 * 判断一个路径是否是邮件链接
 *
 * @param path 待判断的路径
 */
export declare const isMailto: (path: string) => boolean;
/**
 * 判断一个路径是否是电话链接
 *
 * @param path 待判断的路径
 */
export declare const isTel: (path: string) => boolean;
/**
 * 确保路径有合理的后缀
 *
 * @param path 待处理的路径
 */
export declare const ensureExt: (path: string) => string;
/**
 * 确保路径以斜线结尾
 *
 * @param path 待处理的路径
 */
export declare const ensureEndingSlash: (path: string) => string;
/**
 * 判断当前路由是否可以匹配指定链接
 *
 * @param route 当前路由
 * @param path 需要判断的链接
 */
export declare const isActive: (route: Route, path: string) => boolean;
/**
 * 处理路径
 * @param relative 需要处理的路径
 * @param base 部署的基础路径
 * @param append 是否直接添加
 */
export declare const resolvePath: (relative: string, base: string, append?: boolean | undefined) => string;
