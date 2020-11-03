import { ArticleMixin, TimelineMixin } from "@theme/util/articleMixin";
declare const BloggerInfo_base: import("vue-class-component/lib/declarations").VueClass<ArticleMixin & TimelineMixin>;
export default class BloggerInfo extends BloggerInfo_base {
    private get blogConfig();
    private get bloggerName();
    private get bloggerAvatar();
    private get hasIntro();
    private get i18n();
    private navigate;
    private jumpIntro;
}
export {};
