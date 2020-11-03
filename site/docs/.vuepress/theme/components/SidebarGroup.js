import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import DropdownTransition from "@theme/components/DropdownTransition.vue";
import { isActive } from "@theme/util/path";
let SidebarGroup = class SidebarGroup extends Vue {
    constructor() {
        super(...arguments);
        this.isActive = isActive;
    }
    getIcon(icon) {
        const { iconPrefix } = this.$themeConfig;
        return this.$themeConfig.sidebarIcon !== false && icon
            ? `${iconPrefix === "" ? "" : iconPrefix || "icon-"}${icon}`
            : "";
    }
    beforeCreate() {
        // eslint-disable-next-line
        this.$options.components.SidebarLinks = require("@theme/components/SidebarLinks.vue").default;
    }
};
__decorate([
    Prop({ type: Object, default: () => ({}) })
], SidebarGroup.prototype, "item", void 0);
__decorate([
    Prop(Boolean)
], SidebarGroup.prototype, "open", void 0);
__decorate([
    Prop(Number)
], SidebarGroup.prototype, "depth", void 0);
SidebarGroup = __decorate([
    Component({ components: { DropdownTransition } })
], SidebarGroup);
export default SidebarGroup;
