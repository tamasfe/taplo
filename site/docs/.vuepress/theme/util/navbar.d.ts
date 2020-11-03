import { HopeNavBarConfigItem } from "@mr-hope/vuepress-shared-utils";
export interface NavBarConfigItem extends HopeNavBarConfigItem {
    type: "link" | "links";
    items: NavBarConfigItem[];
}
export declare const resolveNavLinkItem: (navbarLink: HopeNavBarConfigItem, beforeprefix?: string) => NavBarConfigItem;
