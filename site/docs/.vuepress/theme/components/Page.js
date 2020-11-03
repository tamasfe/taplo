import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import Anchor from "@theme/components/Anchor.vue";
import Comment from "@Comment";
import MyTransition from "@theme/components/MyTransition.vue";
import PageEdit from "@theme/components/PageEdit.vue";
import PageFooter from "@theme/components/PageFooter.vue";
import PageInfo from "@PageInfo";
import PageNav from "@theme/components/PageNav.vue";
import Password from "@theme/components/Password.vue";
let Page = class Page extends Vue {
    constructor() {
        super(...arguments);
        /** 用户输入的密码 */
        this.password = "";
    }
    /** 是否启用评论 */
    commentEnable() {
        return this.$themeConfig.comment !== false;
    }
    /** 当前页面密码 */
    get pagePassword() {
        /** 页面当前密码 */
        const { password } = this.$frontmatter;
        return typeof password === "number"
            ? password.toString()
            : typeof password === "string"
                ? password
                : "";
    }
    /** 当前页面解密状态 */
    get pageDescrypted() {
        return this.password === this.pagePassword;
    }
};
__decorate([
    Prop({ type: Array, default: () => [] })
], Page.prototype, "sidebarItems", void 0);
__decorate([
    Prop({ type: Array, default: () => [] })
], Page.prototype, "headers", void 0);
Page = __decorate([
    Component({
        components: {
            Anchor,
            Comment,
            MyTransition,
            PageEdit,
            PageFooter,
            PageInfo,
            PageNav,
            Password,
        },
    })
], Page);
export default Page;
