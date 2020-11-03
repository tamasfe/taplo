import { ArticleMixin } from "@theme/util/articleMixin";
declare const ArticleList_base: import("vue-class-component/lib/declarations").VueClass<ArticleMixin>;
export default class ArticleList extends ArticleList_base {
    /** 当前页面 */
    private currentPage;
    /** 文章列表 */
    private articleList;
    /** 博客配置 */
    private get blogConfig();
    /** 文章分页 */
    private get $paginationArticles();
    /** 每页文章数 */
    private get articlePerPage();
    /** 当前页面的文章 */
    private get articles();
    /** 更新文章列表 */
    private getArticleList;
    private mounted;
    /** 在路径发生改变时更新文章列表 */
    private onRouteUpdate;
    private onPageChange;
}
export {};
