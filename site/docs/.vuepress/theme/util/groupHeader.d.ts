import { PageHeader } from "@mr-hope/vuepress-types";
/** 侧边栏标题配置 */
export interface SidebarHeader extends PageHeader {
    /** 子标题 */
    children?: PageHeader[];
}
/**
 * 将低等级的标题置于 h2 的 children 中
 *
 * @param headers
 */
declare const groupHeaders: (headers: PageHeader[]) => SidebarHeader[];
export default groupHeaders;
