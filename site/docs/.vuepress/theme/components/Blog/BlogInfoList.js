import { __decorate } from "tslib";
import { ArticleMixin, StickyMixin } from "@theme/util/articleMixin";
import { Component, Mixins } from "vue-property-decorator";
import { i18n } from "@mr-hope/vuepress-shared-utils";
import ArticleIconFill from "@mr-hope/vuepress-shared-utils/icons/ArticleIconFill.vue";
import ArticleList from "@theme/components/Blog/ArticleList.vue";
import CategoryIcon from "@mr-hope/vuepress-shared-utils/icons/CategoryIcon.vue";
import CategoryList from "@theme/components/Blog/CategoryList.vue";
import MyTransition from "@theme/components/MyTransition.vue";
import TagIcon from "@mr-hope/vuepress-shared-utils/icons/TagIcon.vue";
import TagList from "@theme/components/Blog/TagList.vue";
import TimeIcon from "@mr-hope/vuepress-shared-utils/icons/TimeIcon.vue";
import Timeline from "@theme/components/Blog/Timeline.vue";
import TimelineList from "@theme/components/Blog/TimelineList.vue";
let BlogInfo = class BlogInfo extends Mixins(ArticleMixin, StickyMixin) {
    constructor() {
        super(...arguments);
        this.active = "category";
    }
    get i18n() {
        return this.$themeLocaleConfig.blog || i18n.getDefaultLocale().blog;
    }
    setActive(name) {
        this.active = name;
    }
};
BlogInfo = __decorate([
    Component({
        components: {
            ArticleIconFill,
            ArticleList,
            CategoryIcon,
            CategoryList,
            MyTransition,
            TagIcon,
            TagList,
            TimeIcon,
            Timeline,
            TimelineList,
        },
    })
], BlogInfo);
export default BlogInfo;
