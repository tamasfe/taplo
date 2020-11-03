import { __decorate } from "tslib";
import { Component, Mixins } from "vue-property-decorator";
import MyTransition from "@theme/components/MyTransition.vue";
import { TimelineMixin } from "@theme/util/articleMixin";
import { i18n } from "@mr-hope/vuepress-shared-utils";
let Timeline = class Timeline extends Mixins(TimelineMixin) {
    /** 提示文字 */
    get hint() {
        return ((this.$themeConfig.blog && this.$themeConfig.blog.timeline) ||
            this.$themeLocaleConfig.blog.timelineText ||
            i18n.getDefaultLocale().blog.timelineText);
    }
    navigate(url) {
        void this.$router.push(url);
    }
};
Timeline = __decorate([
    Component({ components: { MyTransition } })
], Timeline);
export default Timeline;
