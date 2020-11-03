import { Vue } from "vue-property-decorator";
import { PageComputed } from "@mr-hope/vuepress-types";
export interface TimelineItem {
    year: number;
    articles: PageComputed[];
}
export declare class ArticleMixin extends Vue {
    /** 文章列表 */
    protected get $articles(): PageComputed[];
}
export declare class TimelineMixin extends Vue {
    /** 时间轴项目 */
    protected get $timelineItems(): PageComputed[];
    /** 时间轴列表 */
    protected get $timeline(): TimelineItem[];
}
export declare class StickyMixin extends Vue {
    /** 文章列表 */
    protected get $stickArticles(): PageComputed[];
}
