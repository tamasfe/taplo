import { ArticleMixin, StickyMixin } from "@theme/util/articleMixin";
declare const BlogPage_base: import("vue-class-component/lib/declarations").VueClass<ArticleMixin & StickyMixin>;
export default class BlogPage extends BlogPage_base {
    private get articleListText();
    private heroHeight;
    /** 是否显示文章 */
    private get displayArticles();
    /** 组件名称 */
    private get componentName();
}
export {};
