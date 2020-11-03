import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import { filterArticle, getDate, sortArticle } from "./article";
let ArticleMixin = class ArticleMixin extends Vue {
    /** 文章列表 */
    get $articles() {
        const { pages } = this.$site;
        // 先过滤再排序
        return sortArticle(filterArticle(pages));
    }
};
ArticleMixin = __decorate([
    Component
], ArticleMixin);
export { ArticleMixin };
let TimelineMixin = class TimelineMixin extends Vue {
    /** 时间轴项目 */
    get $timelineItems() {
        const { pages } = this.$site;
        // 先过滤再排序
        return sortArticle(filterArticle(pages, (frontmatter) => (frontmatter.time || frontmatter.date) &&
            frontmatter.timeline !== false));
    }
    /** 时间轴列表 */
    get $timeline() {
        const timelineItems = [];
        // 先过滤再排序
        this.$timelineItems.forEach((article) => {
            const { frontmatter: { date, time = date }, } = article;
            const [year, month, day] = getDate(time);
            if (year && month && day) {
                if (!timelineItems[0] || timelineItems[0].year !== year)
                    timelineItems.unshift({ year, articles: [] });
                article.frontmatter.parsedDate = `${month}-${day}`;
                timelineItems[0].articles.push(article);
            }
        });
        return timelineItems.reverse();
    }
};
TimelineMixin = __decorate([
    Component
], TimelineMixin);
export { TimelineMixin };
let StickyMixin = class StickyMixin extends Vue {
    /** 文章列表 */
    get $stickArticles() {
        const { pages } = this.$site;
        // 先过滤再排序
        return sortArticle(filterArticle(pages, (frontmatter) => Boolean(frontmatter.sticky)));
    }
};
StickyMixin = __decorate([
    Component
], StickyMixin);
export { StickyMixin };
