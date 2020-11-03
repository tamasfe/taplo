import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import MediaLinks from "@theme/components/MediaLinks.vue";
let PageFooter = class PageFooter extends Vue {
    get footerConfig() {
        return this.$themeConfig.footer || {};
    }
    /** 显示页脚 */
    get display() {
        const { copyrightText, footer, medialink } = this.$page.frontmatter;
        return (footer !== false &&
            Boolean(copyrightText || footer || medialink || this.footerConfig.display));
    }
    /** 页脚内容 */
    get footerContent() {
        const { footer } = this.$page.frontmatter;
        return footer === false
            ? false
            : typeof footer === "string"
                ? footer
                : this.footerConfig.content || "";
    }
    /** 版权信息 */
    get copyright() {
        return this.$frontmatter.copyrightText === false
            ? false
            : this.$frontmatter.copyrightText ||
                this.footerConfig.copyright ||
                (this.$themeConfig.author
                    ? `Copyright © 2020 ${this.$themeConfig.author}`
                    : "");
    }
};
PageFooter = __decorate([
    Component({ components: { MediaLinks } })
], PageFooter);
export default PageFooter;
