import { __decorate } from "tslib";
import { Component, Mixins } from "vue-property-decorator";
import MyTransition from "@theme/components/MyTransition.vue";
import TimeIcon from "@mr-hope/vuepress-shared-utils/icons/TimeIcon.vue";
import { TimelineMixin } from "@theme/util/articleMixin";
import { i18n } from "@mr-hope/vuepress-shared-utils";
let TimelineList = class TimelineList extends Mixins(TimelineMixin) {
    get timeline() {
        return (this.$themeLocaleConfig.blog.timeline ||
            i18n.getDefaultLocale().blog.timeline);
    }
    navigate(url) {
        void this.$router.push(url);
    }
};
TimelineList = __decorate([
    Component({ components: { MyTransition, TimeIcon } })
], TimelineList);
export default TimelineList;
