import { __decorate } from "tslib";
import { Component, Mixins, Watch } from "vue-property-decorator";
import ArticleItem from "@theme/components/Blog/ArticleItem.vue";
import { ArticleMixin } from "@theme/util/articleMixin";
import MyTransition from "@theme/components/MyTransition.vue";
import Pagination from "@mr-hope/vuepress-plugin-components/src/Pagination.vue";
import { generatePagination } from "@theme/util/article";
let ArticleList = class ArticleList extends Mixins(ArticleMixin) {
    constructor() {
        super(...arguments);
        /** 当前页面 */
        this.currentPage = 1;
        /** 文章列表 */
        this.articleList = [];
    }
    /** 博客配置 */
    get blogConfig() {
        return this.$themeConfig.blog || {};
    }
    /** 文章分页 */
    get $paginationArticles() {
        return generatePagination(this.$articles);
    }
    /** 每页文章数 */
    get articlePerPage() {
        return this.blogConfig.perPage || 10;
    }
    /** 当前页面的文章 */
    get articles() {
        return this.articleList.slice((this.currentPage - 1) * this.articlePerPage, this.currentPage * this.articlePerPage);
    }
    /** 更新文章列表 */
    getArticleList() {
        try {
            return this.$pagination
                ? this.$pagination._matchedPages
                : this.$articles;
        }
        catch (err) {
            return this.$articles;
        }
    }
    mounted() {
        this.articleList = this.getArticleList();
    }
    /** 在路径发生改变时更新文章列表 */
    onRouteUpdate(to, from) {
        if (to.path !== from.path) {
            this.articleList = this.getArticleList();
            // 将页面重置为 1
            this.currentPage = 1;
        }
    }
    onPageChange() {
        // 滚动到列表顶部
        const distance = document.querySelector("#article").getBoundingClientRect()
            .top + window.scrollY;
        setTimeout(() => {
            window.scrollTo(0, distance);
        }, 100);
    }
};
__decorate([
    Watch("$route")
], ArticleList.prototype, "onRouteUpdate", null);
__decorate([
    Watch("currentPage")
], ArticleList.prototype, "onPageChange", null);
ArticleList = __decorate([
    Component({ components: { ArticleItem, MyTransition, Pagination } })
], ArticleList);
export default ArticleList;
