import { PageComputed, PageFrontmatter } from "@mr-hope/vuepress-types";
/** 处理日期 */
export declare const getDate: (dateString: string) => (number | undefined)[];
/**
 * 日期比较
 * @param dateA 比较的日期A
 * @param dateB 比较的日期B
 */
export declare const compareDate: (dataA: string | undefined, dataB: string | undefined) => number;
/**
 * 过滤文章
 *
 * @param pages 页面
 * @param filterFunc 额外的过滤函数
 */
export declare const filterArticle: (pages: PageComputed[], filterFunc?: ((frontmatter: PageFrontmatter) => boolean) | undefined) => PageComputed[];
/** 排序文章 */
export declare const sortArticle: (pages: PageComputed[]) => PageComputed[];
export declare const generatePagination: (pages: PageComputed[], perPage?: number) => PageComputed[][];
