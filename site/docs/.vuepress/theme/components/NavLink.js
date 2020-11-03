import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import { ensureExt, isExternal, isMailto, isTel } from "@theme/util/path";
let NavLink = class NavLink extends Vue {
    get link() {
        return ensureExt(this.item.link);
    }
    get iconPrefix() {
        const { iconPrefix } = this.$themeConfig;
        return iconPrefix === "" ? "" : iconPrefix || "icon-";
    }
    get active() {
        return this.link === this.$route.path;
    }
    get isNonHttpURI() {
        return isMailto(this.link) || isTel(this.link);
    }
    get isBlankTarget() {
        return this.target === "_blank";
    }
    get isInternal() {
        return !isExternal(this.link) && !this.isBlankTarget;
    }
    get target() {
        if (this.isNonHttpURI)
            return null;
        if (this.item.target)
            return this.item.target;
        return isExternal(this.link) ? "_blank" : "";
    }
    get rel() {
        if (this.isNonHttpURI)
            return null;
        if (this.item.rel === false)
            return null;
        if (this.item.rel)
            return this.item.rel;
        return this.isBlankTarget ? "noopener noreferrer" : null;
    }
    focusoutAction() {
        this.$emit("focusout");
    }
};
__decorate([
    Prop({ type: Object, required: true })
], NavLink.prototype, "item", void 0);
NavLink = __decorate([
    Component
], NavLink);
export default NavLink;
