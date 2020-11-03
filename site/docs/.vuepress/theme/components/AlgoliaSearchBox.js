import { __decorate } from "tslib";
import { Component, Prop, Vue, Watch } from "vue-property-decorator";
let AlgoliaSearchBox = class AlgoliaSearchBox extends Vue {
    constructor() {
        super(...arguments);
        this.placeholder = "";
    }
    onLangChange(newValue) {
        this.update(this.options, newValue);
    }
    onOptionsChange(newValue) {
        this.update(newValue, this.$lang);
    }
    mounted() {
        this.initialize(this.options, this.$lang);
        this.placeholder =
            this.$site.themeConfig.searchPlaceholder || "";
    }
    initialize(userOptions, lang) {
        void Promise.all([
            import(
            /* webpackChunkName: "docsearch" */ "docsearch.js/dist/cdn/docsearch.min.js"),
            import(
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            /* webpackChunkName: "docsearch" */ "docsearch.js/dist/cdn/docsearch.min.css"),
        ]).then(([docsearch]) => {
            // eslint-disable-next-line
            docsearch.default({
                ...userOptions,
                inputSelector: "#algolia-search-input",
                // #697 Make docsearch work well at i18n mode.
                algoliaOptions: {
                    facetFilters: [`lang:${lang}`].concat(userOptions.facetFilters || []),
                },
                handleSelected: (_input, _event, suggestion) => {
                    const { pathname, hash } = new URL(suggestion.url);
                    const routepath = pathname.replace(this.$site.base, "/");
                    void this.$router.push(`${routepath}${decodeURIComponent(hash)}`);
                },
            });
        });
    }
    update(options, lang) {
        this.$el.innerHTML =
            '<input id="algolia-search-input" class="search-query">';
        this.initialize(options, lang);
    }
};
__decorate([
    Prop({ type: Object, required: true })
], AlgoliaSearchBox.prototype, "options", void 0);
__decorate([
    Watch("$lang")
], AlgoliaSearchBox.prototype, "onLangChange", null);
__decorate([
    Watch("options")
], AlgoliaSearchBox.prototype, "onOptionsChange", null);
AlgoliaSearchBox = __decorate([
    Component
], AlgoliaSearchBox);
export default AlgoliaSearchBox;
