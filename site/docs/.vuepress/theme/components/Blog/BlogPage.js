import { __decorate } from "tslib";
import { ArticleMixin, StickyMixin } from "@theme/util/articleMixin";
import { Component, Mixins } from "vue-property-decorator";
import ArticleList from "@theme/components/Blog/ArticleList.vue";
import BlogHero from "@theme/components/Blog/BlogHero.vue";
import BlogInfo from "@BlogInfo";
import CategoryList from "@theme/components/Blog/CategoryList.vue";
import MyTransition from "@theme/components/MyTransition.vue";
import PageFooter from "@theme/components/PageFooter.vue";
import ProjectList from "@theme/components/Blog/ProjectList.vue";
import TagList from "@theme/components/Blog/TagList.vue";
import Timeline from "@theme/components/Blog/Timeline.vue";
import TimelineList from "@theme/components/Blog/TimelineList.vue";
import { i18n } from "@mr-hope/vuepress-shared-utils";
let BlogPage = class BlogPage extends Mixins(ArticleMixin, StickyMixin) {
    get articleListText() {
        return (this.$themeLocaleConfig.blog || i18n.getDefaultLocale().blog)
            .articleList;
    }
    heroHeight() {
        return document.querySelector(".blog-hero").clientHeight;
    }
    /** 是否显示文章 */
    get displayArticles() {
        const { path } = this.$route;
        return !path.includes("/timeline");
    }
    /** 组件名称 */
    get componentName() {
        const pathName = this.$route.path.split("/")[1];
        if (["category", "tag"].includes(pathName))
            return `${pathName}List`;
        else if (pathName === "timeline")
            return pathName;
        return "";
    }
};
BlogPage = __decorate([
    Component({
        components: {
            ArticleList,
            BlogHero,
            BlogInfo,
            CategoryList,
            MyTransition,
            PageFooter,
            ProjectList,
            TagList,
            Timeline,
            TimelineList,
        },
    })
], BlogPage);
export default BlogPage;
