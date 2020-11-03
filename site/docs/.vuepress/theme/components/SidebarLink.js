import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import { groupSidebarHeaders, } from "@theme/util/sidebar";
import { hashRE, isActive } from "@theme/util/path";
/** 渲染图标 */
const renderIcon = (h, icon) => icon
    ? h("i", {
        class: ["iconfont", icon],
        style: "margin-right: 0.2em;",
    })
    : null;
/** 渲染链接 */
const renderLink = (h, { icon = "", text, link, active }) => h("RouterLink", {
    props: {
        to: link,
        activeClass: "",
        exactActiveClass: "",
    },
    class: {
        active,
        "sidebar-link": true,
    },
}, [renderIcon(h, icon), text]);
/** 渲染外部链接 */
const renderExternal = (h, { path, title = path }) => h("a", {
    attrs: {
        href: path,
        target: "_blank",
        rel: "noopener noreferrer",
    },
    class: { "sidebar-link": true },
}, [title, h("OutboundLink")]);
/** 渲染子项 */
const renderChildren = (h, { children, path, route, maxDepth, depth = 1 }) => {
    if (!children || depth > maxDepth)
        return null;
    return h("ul", { class: "sidebar-sub-headers" }, children.map((child) => {
        const active = isActive(route, `${path}#${child.slug}`);
        return h("li", { class: "sidebar-sub-header" }, [
            renderLink(h, {
                text: child.title,
                link: `${path}#${child.slug}`,
                active,
            }),
            renderChildren(h, {
                children: child.children || false,
                path,
                route,
                maxDepth,
                depth: depth + 1,
            }),
        ]);
    }));
};
let SidebarLink = class SidebarLink extends Vue {
};
__decorate([
    Prop({ type: Object, default: () => ({}) })
], SidebarLink.prototype, "item", void 0);
SidebarLink = __decorate([
    Component({
        functional: true,
        render(h, { parent: { $page, $route, $themeConfig, $themeLocaleConfig }, props }) {
            /** 当前渲染项目配置 */
            const { item } = props;
            // 当前配置未获取成功
            if (item.type === "error")
                return null;
            // 外部链接侧边栏项
            if (item.type === "external")
                return renderExternal(h, item);
            /*
             * Use custom active class matching logic
             * Due to edge case of paths ending with / + hash
             */
            const selfActive = isActive($route, item.path);
            /** 当前渲染项目的激活状态 */
            const active = 
            // 如果是标题侧边栏的话，其中一个子标题匹配也需要激活
            item.type === "header"
                ? selfActive ||
                    (item.children || []).some((child) => isActive($route, `${item.basePath}#${child.slug}`))
                : selfActive;
            /** 最大显示深度 */
            const maxDepth = $page.frontmatter.sidebarDepth ||
                $themeLocaleConfig.sidebarDepth ||
                $themeConfig.sidebarDepth ||
                2;
            // 如果是标题侧边栏
            if (item.type === "header")
                return [
                    renderLink(h, {
                        text: item.title || item.path,
                        link: item.path,
                        active,
                    }),
                    renderChildren(h, {
                        children: item.children || false,
                        path: item.basePath,
                        route: $route,
                        maxDepth,
                    }),
                ];
            /** 是否显示所有标题 */
            const displayAllHeaders = $themeLocaleConfig.displayAllHeaders ||
                $themeConfig.displayAllHeaders;
            const link = renderLink(h, {
                icon: $themeConfig.sidebarIcon !== false && item.frontmatter.icon
                    ? `${$themeConfig.iconPrefix === ""
                        ? ""
                        : $themeConfig.iconPrefix || "icon-"}${item.frontmatter.icon}`
                    : "",
                text: item.title || item.path,
                link: item.path,
                active,
            });
            if ((active || displayAllHeaders) &&
                item.headers &&
                !hashRE.test(item.path)) {
                const children = groupSidebarHeaders(item.headers);
                return [
                    link,
                    renderChildren(h, {
                        children,
                        path: item.path,
                        route: $route,
                        maxDepth,
                    }),
                ];
            }
            return link;
        },
    })
], SidebarLink);
export default SidebarLink;
