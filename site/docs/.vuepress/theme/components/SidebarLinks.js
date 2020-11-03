import { __decorate } from "tslib";
import { Component, Prop, Vue, Watch } from "vue-property-decorator";
import SidebarGroup from "@theme/components/SidebarGroup.vue";
import SidebarLink from "@theme/components/SidebarLink.vue";
import { isActive } from "@theme/util/path";
/** 当前项目是否激活 */
const descendantIsActive = (route, item) => {
    if (item.type === "group")
        return item.children.some((child) => {
            if (child.type === "group")
                return descendantIsActive(route, child);
            return child.type === "page" && isActive(route, child.path);
        });
    return false;
};
/** 打开的侧边栏组的索引值 */
const resolveOpenGroupIndex = (route, items) => {
    for (let i = 0; i < items.length; i++)
        if (descendantIsActive(route, items[i]))
            return i;
    return -1;
};
let SidebarLinks = class SidebarLinks extends Vue {
    constructor() {
        super(...arguments);
        this.openGroupIndex = 0;
    }
    refreshIndex() {
        const index = resolveOpenGroupIndex(this.$route, this.items);
        if (index > -1)
            this.openGroupIndex = index;
    }
    toggleGroup(index) {
        this.openGroupIndex = index === this.openGroupIndex ? -1 : index;
    }
    isActive(page) {
        return isActive(this.$route, page.regularPath);
    }
    created() {
        this.refreshIndex();
    }
    onRouteUpdate() {
        this.refreshIndex();
    }
};
__decorate([
    Prop(Array)
], SidebarLinks.prototype, "items", void 0);
__decorate([
    Prop(Number)
], SidebarLinks.prototype, "depth", void 0);
__decorate([
    Watch("$route")
], SidebarLinks.prototype, "onRouteUpdate", null);
SidebarLinks = __decorate([
    Component({ components: { SidebarGroup, SidebarLink } })
], SidebarLinks);
export default SidebarLinks;
