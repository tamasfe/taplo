import { PageComputed, SiteData } from "@mr-hope/vuepress-types";
import { SidebarHeader } from "./groupHeader";
export { SidebarHeader } from "./groupHeader";
export interface SidebarHeaderItem extends SidebarHeader {
    type: "header";
    basePath: string;
    path: string;
}
export interface SidebarAutoItem {
    type: "group";
    /** 分组的标题 */
    title: string;
    /** 页面图标 */
    icon?: string;
    /** 页面内的标题 */
    children: SidebarHeaderItem[];
    collapsable: false;
    path: "";
}
export declare const groupSidebarHeaders: (headers: import("@mr-hope/vuepress-types").PageHeader[]) => SidebarHeader[];
/** 外部链接侧边栏项 */
export interface SidebarExternalItem {
    /** 标题 */
    title?: string;
    /** 图标 */
    icon?: string;
    /** 类型 */
    type: "external";
    /** 链接路径 */
    path: string;
}
/** 页面侧边栏项 */
export interface SidebarPageItem extends PageComputed {
    type: "page";
    /** 图标 */
    icon?: string;
    /** 路径 */
    path: string;
}
/** 分组侧边栏项 */
export interface SidebarGroupItem {
    type: "group";
    /** 分组的标题 */
    title: string;
    /** 可折叠，默认为 true */
    collapsable: boolean;
    /** 侧边栏深度，默认为 1 */
    sidebarDepth?: number;
    /** 分组的图标 */
    icon?: string;
    /** 当前分组的路径前缀 */
    prefix?: string;
    /** 当前侧边栏的子项 */
    children: SidebarItem[];
    [props: string]: unknown;
}
export interface SidebarErrorItem {
    type: "error";
    path: string;
}
/**
 * 处理侧边栏项，为其合并页面配置
 *
 * @param pages
 * @param path 配置中的路径
 */
export declare const resolvePageforSidebar: (pages: PageComputed[], path: string) => SidebarPageItem | SidebarExternalItem | SidebarErrorItem;
export declare type SidebarItem = SidebarAutoItem | SidebarErrorItem | SidebarExternalItem | SidebarGroupItem | SidebarPageItem;
/**
 * 获得当前页面的侧边栏对象
 */
export declare const resolveSidebarItems: (page: PageComputed, site: SiteData, localePath: string) => SidebarItem[];
