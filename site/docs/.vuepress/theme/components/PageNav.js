import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import { resolvePageforSidebar, } from "@theme/util/sidebar";
import NextIcon from "@mr-hope/vuepress-shared-utils/icons/NextIcon.vue";
import PrevIcon from "@mr-hope/vuepress-shared-utils/icons/PrevIcon.vue";
import { resolvePath } from "@theme/util/path";
const getSidebarItems = (items, result) => {
    for (const item of items)
        if (item.type === "group")
            getSidebarItems(item.children || [], result);
        else
            result.push(item);
};
const find = (page, items, offset) => {
    const result = [];
    getSidebarItems(items, result);
    for (let i = 0; i < result.length; i++) {
        const cur = result[i];
        if (cur.type === "page" && cur.path === decodeURIComponent(page.path))
            return result[i + offset];
    }
    return false;
};
/** 处理页面链接 */
const resolvePageLink = (linkType, { themeConfig, page, route, site, sidebarItems }) => {
    const themeLinkConfig = themeConfig[`${linkType}Links`];
    const pageLinkConfig = page.frontmatter[linkType];
    if (themeLinkConfig === false || pageLinkConfig === false)
        return false;
    if (typeof pageLinkConfig === "string")
        return resolvePageforSidebar(site.pages, resolvePath(pageLinkConfig, route.path));
    return find(page, sidebarItems, linkType === "prev" ? -1 : 1);
};
let PageNav = class PageNav extends Vue {
    get prev() {
        return resolvePageLink("prev", {
            sidebarItems: this.sidebarItems,
            themeConfig: this.$themeConfig,
            page: this.$page,
            route: this.$route,
            site: this.$site,
        });
    }
    get next() {
        return resolvePageLink("next", {
            sidebarItems: this.sidebarItems,
            themeConfig: this.$themeConfig,
            page: this.$page,
            route: this.$route,
            site: this.$site,
        });
    }
};
__decorate([
    Prop(Array)
], PageNav.prototype, "sidebarItems", void 0);
PageNav = __decorate([
    Component({ components: { NextIcon, PrevIcon } })
], PageNav);
export default PageNav;
