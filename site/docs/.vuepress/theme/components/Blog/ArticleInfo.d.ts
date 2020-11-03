import { Vue } from "vue-property-decorator";
export default class ArticleInfo extends Vue {
    private readonly article;
    /** 作者 */
    private get author();
    /** 发表时间 */
    private get time();
    /** 标签 */
    private get tags();
    private get readtime();
    private get authorText();
    private get timeText();
    private get tagText();
    private get readingTimeText();
}
