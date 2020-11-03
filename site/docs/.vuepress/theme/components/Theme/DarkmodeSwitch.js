import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import AutoIcon from "@mr-hope/vuepress-shared-utils/icons/AutoIcon.vue";
import DarkIcon from "@mr-hope/vuepress-shared-utils/icons/DarkIcon.vue";
import LightIcon from "@mr-hope/vuepress-shared-utils/icons/LightIcon.vue";
import { dom } from "@mr-hope/vuepress-shared-utils";
let DarkmodeSwitch = class DarkmodeSwitch extends Vue {
    constructor() {
        super(...arguments);
        this.darkmode = "auto";
    }
    /** darkmode status */
    get darkmodeConfig() {
        return this.$themeConfig.darkmode || "auto-switch";
    }
    mounted() {
        /** 深色模式 */
        this.darkmode =
            localStorage.getItem("darkmode") ||
                "auto";
        if (this.darkmodeConfig === "auto-switch")
            if (this.darkmode === "auto")
                this.setDarkmode("auto");
            else
                this.setDarkmode(this.darkmode);
        // 自动模式
        else if (this.darkmodeConfig === "auto")
            this.setDarkmode("auto");
        // 切换模式
        else if (this.darkmodeConfig === "switch")
            this.setDarkmode(this.darkmode);
        // 被禁用
        else
            this.setDarkmode("off");
    }
    /** 设置夜间模式 */
    setDarkmode(status) {
        if (status === "on")
            this.toggleDarkmode(true);
        else if (status === "off")
            this.toggleDarkmode(false);
        else {
            const isDarkMode = window.matchMedia("(prefers-color-scheme: dark)")
                .matches;
            const isLightMode = window.matchMedia("(prefers-color-scheme: light)")
                .matches;
            window.matchMedia("(prefers-color-scheme: dark)").addListener((event) => {
                if (event.matches)
                    this.toggleDarkmode(true);
            });
            window
                .matchMedia("(prefers-color-scheme: light)")
                .addListener((event) => {
                if (event.matches)
                    this.toggleDarkmode(false);
            });
            if (isDarkMode)
                this.toggleDarkmode(true);
            else if (isLightMode)
                this.toggleDarkmode(false);
            else {
                const timeHour = new Date().getHours();
                this.toggleDarkmode(timeHour < 6 || timeHour >= 18);
            }
        }
        this.darkmode = status;
        localStorage.setItem("darkmode", status);
    }
    /** 切换深色模式 */
    toggleDarkmode(isDarkmode) {
        const classes = document.body.classList;
        if (isDarkmode)
            dom.changeClass(classes, ["theme-dark"], ["theme-light"]);
        else
            dom.changeClass(classes, ["theme-light"], ["theme-dark"]);
    }
};
DarkmodeSwitch = __decorate([
    Component({ components: { AutoIcon, DarkIcon, LightIcon } })
], DarkmodeSwitch);
export default DarkmodeSwitch;
