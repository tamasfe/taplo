import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
let Clipboard = class Clipboard extends Vue {
    constructor() {
        super(...arguments);
        this.location = "";
    }
    get copyright() {
        /** 作者 */
        const { author } = this.$themeConfig;
        /** 内容 */
        const content = {
            "zh-CN": `${this.html}\n-----\n${author ? `著作权归${author}所有。\n` : ""}链接: ${this.location}`,
            "en-US": `${this.html}\n-----\n${author ? `Copyright by ${author}.\n` : ""}Link: ${this.location}`,
            "vi-VN": `${this.html}\n-----\n${author ? `bản quyền bởi ${author}.\n` : ""}Liên kết: ${this.location}`,
        };
        return content[this.lang];
    }
    created() {
        if (typeof window !== "undefined")
            this.location = window.location.toString();
    }
};
__decorate([
    Prop({ type: String, default: "" })
], Clipboard.prototype, "html", void 0);
__decorate([
    Prop({ type: String, default: "en-US" })
], Clipboard.prototype, "lang", void 0);
Clipboard = __decorate([
    Component
], Clipboard);
export default Clipboard;
