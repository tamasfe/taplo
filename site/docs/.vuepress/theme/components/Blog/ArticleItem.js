import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import ArticleInfo from "@theme/components/Blog/ArticleInfo.vue";
import LockIcon from "@mr-hope/vuepress-shared-utils/icons/LockIcon.vue";
import StickyIcon from "@mr-hope/vuepress-shared-utils/icons/StickyIcon.vue";
import { pathHitKeys } from "@theme/util/encrypt";
let ArticleItem = class ArticleItem extends Vue {
    /** 文章是否加密 */
    get isEncrypted() {
        return (pathHitKeys(this.$themeConfig.encrypt, this.article.path).length !== 0 ||
            Boolean(this.article.frontmatter.password));
    }
};
__decorate([
    Prop({ type: Object, required: true })
], ArticleItem.prototype, "article", void 0);
ArticleItem = __decorate([
    Component({ components: { ArticleInfo, LockIcon, StickyIcon } })
], ArticleItem);
export default ArticleItem;
