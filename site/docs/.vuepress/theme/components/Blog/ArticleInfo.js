import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
import { capitalize } from "@mr-hope/vuepress-shared-utils";
import AuthorIcon from "@mr-hope/vuepress-shared-utils/icons/AuthorIcon.vue";
import CalendarIcon from "@mr-hope/vuepress-shared-utils/icons/CalendarIcon.vue";
import CategoryInfo from "@mr-hope/vuepress-plugin-comment/src/CategoryInfo.vue";
import TagInfo from "@mr-hope/vuepress-plugin-comment/src/TagInfo.vue";
import TimeIcon from "@mr-hope/vuepress-shared-utils/icons/TimeIcon.vue";
let ArticleInfo = class ArticleInfo extends Vue {
    /** 作者 */
    get author() {
        return (this.article.frontmatter.author ||
            (this.$themeConfig.author && this.article.frontmatter.author !== false
                ? this.$themeConfig.author
                : ""));
    }
    /** 发表时间 */
    get time() {
        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const { date, time = date } = this.article.frontmatter;
        if (typeof time === "string") {
            if (time.indexOf("T") !== -1) {
                const [dateString, temp] = time.split("T");
                const [times] = temp.split(".");
                return `${dateString} ${times === "00:00:00" ? "" : times}`;
            }
            return time;
        }
        return "";
    }
    /** 标签 */
    get tags() {
        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const { tag, tags = tag } = this.article.frontmatter;
        if (typeof tags === "string")
            return [capitalize(tags)];
        if (Array.isArray(tags))
            return tags.map((item) => capitalize(item));
        return [];
    }
    get readtime() {
        const { minute, time } = READING_TIME_I18N[this.$localePath || "/"];
        return this.article.readingTime.minutes < 1
            ? minute
            : time.replace("$time", Math.round(this.article.readingTime.minutes).toString());
    }
    get authorText() {
        return PAGE_INFO_I18N[this.$localePath || "/"].author;
    }
    get timeText() {
        return PAGE_INFO_I18N[this.$localePath || "/"].time;
    }
    get tagText() {
        return PAGE_INFO_I18N[this.$localePath || "/"].tag;
    }
    get readingTimeText() {
        return PAGE_INFO_I18N[this.$localePath || "/"].readingTime;
    }
};
__decorate([
    Prop(Object)
], ArticleInfo.prototype, "article", void 0);
ArticleInfo = __decorate([
    Component({
        components: {
            AuthorIcon,
            CalendarIcon,
            CategoryInfo,
            TagInfo,
            TimeIcon,
        },
    })
], ArticleInfo);
export default ArticleInfo;
