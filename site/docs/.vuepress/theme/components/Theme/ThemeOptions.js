import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import { i18n } from "@mr-hope/vuepress-shared-utils";
import DarkmodeSwitch from "@theme/components/Theme/DarkmodeSwitch.vue";
/** 默认颜色选择器 */
const defaultPicker = {
    red: "#e74c3c",
    blue: "#3498db",
    green: "#3eaf7c",
    orange: "#f39c12",
    purple: "#8e44ad",
};
let ThemeOptions = class ThemeOptions extends Vue {
    constructor() {
        super(...arguments);
        this.themeColor = {};
        this.isDarkmode = false;
    }
    get text() {
        return (i18n.getLocale(this.$lang).themeColor ||
            i18n.getDefaultLocale().themeColor);
    }
    get themeColorEnabled() {
        return this.$themeConfig.themeColor !== false;
    }
    get switchEnabled() {
        return (this.$themeConfig.darkmode !== "disable" &&
            this.$themeConfig.darkmode !== "auto");
    }
    mounted() {
        /** 所选主题 */
        const theme = localStorage.getItem("theme");
        this.themeColor = {
            list: this.$themeConfig.themeColor
                ? Object.keys(this.$themeConfig.themeColor)
                : Object.keys(defaultPicker),
            picker: this.$themeConfig.themeColor || defaultPicker,
        };
        if (theme)
            this.setTheme(theme);
    }
    /** 设置主题 */
    setTheme(theme) {
        const classes = document.body.classList;
        const themes = this.themeColor.list.map((colorTheme) => `theme-${colorTheme}`);
        if (!theme) {
            localStorage.removeItem("theme");
            classes.remove(...themes);
            return;
        }
        classes.remove(...themes.filter((themeclass) => themeclass !== `theme-${theme}`));
        classes.add(`theme-${theme}`);
        localStorage.setItem("theme", theme);
    }
};
ThemeOptions = __decorate([
    Component({ components: { DarkmodeSwitch } })
], ThemeOptions);
export default ThemeOptions;
