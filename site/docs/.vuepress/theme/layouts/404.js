import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import Common from "@theme/components/Common.vue";
import { i18n } from "@mr-hope/vuepress-shared-utils";
let NotFound = class NotFound extends Vue {
    get i18n() {
        return this.$themeLocaleConfig.error404 || i18n.getDefaultLocale().error404;
    }
    get msg() {
        return this.i18n.hint[Math.floor(Math.random() * this.i18n.hint.length)];
    }
    back() {
        window.history.go(-1);
    }
};
NotFound = __decorate([
    Component({ components: { Common } })
], NotFound);
export default NotFound;
