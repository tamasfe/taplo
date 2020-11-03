import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
let Password = class Password extends Vue {
    constructor() {
        super(...arguments);
        this.password = "";
        this.hasTried = false;
    }
    get isMainPage() {
        return this.$frontmatter.home === true;
    }
    verify() {
        this.hasTried = false;
        this.$emit("password-verify", this.password);
        void Vue.nextTick().then(() => {
            this.hasTried = true;
        });
    }
};
__decorate([
    Prop({ type: Boolean, default: false })
], Password.prototype, "page", void 0);
Password = __decorate([
    Component
], Password);
export default Password;
