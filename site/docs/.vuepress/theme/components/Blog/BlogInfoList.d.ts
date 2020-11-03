import { ArticleMixin, StickyMixin } from "@theme/util/articleMixin";
declare const BlogInfo_base: import("vue-class-component/lib/declarations").VueClass<ArticleMixin & StickyMixin>;
export default class BlogInfo extends BlogInfo_base {
    private active;
    private get i18n();
    private setActive;
}
export {};
