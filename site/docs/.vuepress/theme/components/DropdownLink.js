import { __decorate } from "tslib";
import { Component, Prop, Vue, Watch } from "vue-property-decorator";
import DropdownTransition from "@theme/components/DropdownTransition.vue";
import NavLink from "@theme/components/NavLink.vue";
let DropdownLink = class DropdownLink extends Vue {
    constructor() {
        super(...arguments);
        this.open = false;
    }
    get dropdownAriaLabel() {
        return this.item.ariaLabel || this.item.text;
    }
    get iconPrefix() {
        const { iconPrefix } = this.$themeConfig;
        return iconPrefix === "" ? "" : iconPrefix || "icon-";
    }
    setOpen(value) {
        this.open = value;
    }
    handleDropdown(event) {
        const isTriggerByTab = event.detail === 0;
        if (isTriggerByTab)
            this.setOpen(!this.open);
    }
    isLastItemOfArray(item, array) {
        if (Array.isArray(array))
            return item === array[array.length - 1];
        return false;
    }
    onRouteChange() {
        this.open = false;
    }
};
__decorate([
    Prop({ type: Object, required: true })
], DropdownLink.prototype, "item", void 0);
__decorate([
    Watch("$route")
], DropdownLink.prototype, "onRouteChange", null);
DropdownLink = __decorate([
    Component({ components: { NavLink, DropdownTransition } })
], DropdownLink);
export default DropdownLink;
