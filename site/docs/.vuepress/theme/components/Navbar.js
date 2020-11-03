import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import AlgoliaSearchBox from "@AlgoliaSearchBox";
import NavLinks from "@theme/components/NavLinks.vue";
import SearchBox from "@SearchBox";
import SidebarButton from "@theme/components/SidebarButton.vue";
import ThemeColor from "@ThemeColor";
const css = (el, property) => {
    // NOTE: Known bug, will return 'auto' if style value is 'auto'
    const window = el.ownerDocument.defaultView;
    // `null` means not to return pseudo styles
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    return window.getComputedStyle(el, null)[property];
};
let Navbar = class Navbar extends Vue {
    constructor() {
        super(...arguments);
        this.linksWrapMaxWidth = 0;
    }
    /** Algolia 配置 */
    get algolia() {
        return (this.$themeLocaleConfig.algolia || this.$themeConfig.algolia || false);
    }
    /** 是否使用 Algolia 搜索 */
    get isAlgoliaSearch() {
        return Boolean(this.algolia && this.algolia.apiKey && this.algolia.indexName);
    }
    mounted() {
        // Refer to config.styl
        const MOBILE_DESKTOP_BREAKPOINT = 719;
        const NAVBAR_HORIZONTAL_PADDING = parseInt(css(this.$el, "paddingLeft")) +
            parseInt(css(this.$el, "paddingRight"));
        const handleLinksWrapWidth = () => {
            if (document.documentElement.clientWidth < MOBILE_DESKTOP_BREAKPOINT)
                this.linksWrapMaxWidth = 0;
            else
                this.linksWrapMaxWidth =
                    this.$el.offsetWidth -
                        NAVBAR_HORIZONTAL_PADDING -
                        ((this.$refs.siteInfo &&
                            this.$refs.siteInfo.$el &&
                            this.$refs.siteInfo.$el.offsetWidth) ||
                            0);
        };
        handleLinksWrapWidth();
        window.addEventListener("resize", handleLinksWrapWidth, false);
        window.onorientationchange = () => handleLinksWrapWidth;
    }
};
Navbar = __decorate([
    Component({
        components: {
            AlgoliaSearchBox,
            NavLinks,
            SearchBox,
            SidebarButton,
            ThemeColor,
        },
    })
], Navbar);
export default Navbar;
