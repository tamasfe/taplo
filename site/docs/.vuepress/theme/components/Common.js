import { __decorate } from "tslib";
import { Component, Mixins, Prop } from "vue-property-decorator";
import { resolveSidebarItems, } from "@theme/util/sidebar";
import GlobalEncryptMixin from "@theme/util/globalEncryptMixin";
import Navbar from "@theme/components/Navbar.vue";
import Password from "@theme/components/Password.vue";
import Sidebar from "@theme/components/Sidebar.vue";
let Common = class Common extends Mixins(GlobalEncryptMixin) {
    constructor() {
        super(...arguments);
        this.isSidebarOpen = false;
        this.touchStart = {
            clientX: 0,
            clientY: 0,
        };
    }
    /** 是否应该展示导航栏 */
    get showNavbar() {
        if (this.navbar === false)
            return false;
        const { frontmatter } = this.$page;
        if (frontmatter.navbar === false || this.$themeConfig.navbar === false)
            return false;
        return Boolean(this.$title ||
            this.$themeConfig.logo ||
            this.$themeConfig.repo ||
            this.$themeConfig.nav ||
            this.$themeLocaleConfig.nav);
    }
    /** 是否应该展示侧边栏 */
    get showSidebar() {
        if (this.sidebar === false)
            return false;
        return (!this.$frontmatter.home &&
            this.$frontmatter.sidebar !== false &&
            this.sidebarItems.length !== 0);
    }
    /** 侧边栏内容 */
    get sidebarItems() {
        if (this.sidebar === false)
            return [];
        return resolveSidebarItems(this.$page, this.$site, this.$localePath);
    }
    /** 页面 Class */
    get pageClasses() {
        const userPageClass = this.$page.frontmatter.pageClass;
        return [
            {
                "no-navbar": !this.showNavbar,
                "sidebar-open": this.isSidebarOpen,
                "no-sidebar": !this.showSidebar,
                "has-anchor": this.showAnchor,
            },
            userPageClass,
        ];
    }
    get headers() {
        return this.getHeader(this.sidebarItems);
    }
    get showAnchor() {
        return this.$themeConfig.anchorDisplay !== false && this.headers.length > 0;
    }
    mounted() {
        this.$router.afterEach(() => {
            this.isSidebarOpen = false;
        });
    }
    toggleSidebar(to) {
        this.isSidebarOpen = typeof to === "boolean" ? to : !this.isSidebarOpen;
        this.$emit("toggle-sidebar", this.isSidebarOpen);
    }
    // Side swipe
    onTouchStart(event) {
        this.touchStart = {
            clientX: event.changedTouches[0].clientX,
            clientY: event.changedTouches[0].clientY,
        };
    }
    onTouchEnd(event) {
        const dx = event.changedTouches[0].clientX - this.touchStart.clientX;
        const dy = event.changedTouches[0].clientY - this.touchStart.clientY;
        if (Math.abs(dx) > Math.abs(dy) && Math.abs(dx) > 40)
            if (dx > 0 && this.touchStart.clientX <= 80)
                this.toggleSidebar(true);
            else
                this.toggleSidebar(false);
    }
    getHeader(items) {
        for (let i = 0; i < items.length; i++) {
            const item = items[i];
            if (item.type === "group") {
                const matching = this.getHeader(item.children);
                if (matching.length !== 0)
                    return matching;
            }
            else if (item.type === "page" &&
                item.headers &&
                item.path === this.$route.path)
                return item.headers;
        }
        return [];
    }
};
__decorate([
    Prop({ type: Boolean, default: true })
], Common.prototype, "navbar", void 0);
__decorate([
    Prop({ type: Boolean, default: true })
], Common.prototype, "sidebar", void 0);
Common = __decorate([
    Component({ components: { Password, Sidebar, Navbar } })
], Common);
export default Common;
