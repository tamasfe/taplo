import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import BloggerInfo from "@theme/components/Blog/BloggerInfo.vue";
import NavLinks from "@theme/components/NavLinks.vue";
import SidebarLinks from "@theme/components/SidebarLinks.vue";
let Sidebar = class Sidebar extends Vue {
    get blogConfig() {
        return this.$themeConfig.blog || {};
    }
    get sidebarDisplay() {
        return this.blogConfig.sidebarDisplay || "none";
    }
};
__decorate([
    Prop({ type: Array, required: true })
], Sidebar.prototype, "items", void 0);
Sidebar = __decorate([
    Component({ components: { BloggerInfo, SidebarLinks, NavLinks } })
], Sidebar);
export default Sidebar;
