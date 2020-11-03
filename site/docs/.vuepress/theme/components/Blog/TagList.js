import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import { i18n } from "@mr-hope/vuepress-shared-utils";
import navigate from "@theme/util/navigate";
let TagList = class TagList extends Vue {
    /** 标签列表 */
    get tagList() {
        return [
            {
                name: this.$themeLocaleConfig.blog.allText ||
                    i18n.getDefaultLocale().blog.allText,
                path: "/tag/",
            },
            ...this.$tag.list,
        ];
    }
    /** 是否激活 */
    isActive(name) {
        return (name ===
            ((this.$currentTag && this.$currentTag.key) ||
                this.$themeLocaleConfig.blog.allText ||
                i18n.getDefaultLocale().blog.allText));
    }
    /** 点击标签导航 */
    clickTag(path) {
        navigate(path, this.$router, this.$route);
    }
};
TagList = __decorate([
    Component
], TagList);
export default TagList;
