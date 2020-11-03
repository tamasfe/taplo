import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import { groupSidebarHeaders, } from "@theme/util/sidebar";
import { isActive } from "@theme/util/path";
/** 渲染链接 */
const renderLink = (h, { text, link, active }) => h("RouterLink", {
    props: {
        to: link,
        activeClass: "",
        exactActiveClass: "",
    },
    class: {
        active,
        "anchor-link": true,
    },
}, [h("div", {}, [text])]);
/** 渲染子项 */
const renderChildren = (h, { children, path, route, maxDepth, depth = 2 }) => {
    if (!children || depth > maxDepth)
        return null;
    return h("ul", { class: "anchor-list" }, children.map((child) => {
        const active = isActive(route, `${path}#${child.slug}`);
        return h("li", { class: ["anchor", `anchor${depth}`] }, [
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
let Anchor = class Anchor extends Vue {
};
__decorate([
    Prop({ type: Array, default: () => [] })
], Anchor.prototype, "header", void 0);
Anchor = __decorate([
    Component({
        functional: true,
        render(h, { parent: { $page, $route, $themeConfig, $themeLocaleConfig }, props }) {
            /** 当前渲染项目配置 */
            const { header } = props;
            /** 最大显示深度 */
            const maxDepth = ($page.frontmatter.sidebarDepth ||
                $themeLocaleConfig.sidebarDepth ||
                $themeConfig.sidebarDepth ||
                2) + 1;
            const children = groupSidebarHeaders(header);
            return h("aside", { attrs: { id: "anchor" } }, [
                h("div", { class: "anchor-wrapper" }, [
                    renderChildren(h, {
                        children,
                        path: $route.path,
                        route: $route,
                        maxDepth,
                    }),
                ]),
            ]);
        },
    })
], Anchor);
export default Anchor;
