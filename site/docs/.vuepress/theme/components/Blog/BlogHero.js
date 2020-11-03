import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import MyTransition from "@theme/components/MyTransition.vue";
let BlogHero = class BlogHero extends Vue {
    get heroImageStyle() {
        const defaultStyle = {
            maxHeight: "180px",
            margin: this.$frontmatter.showTitle === false
                ? "6rem auto 1.5rem"
                : "1rem auto",
        };
        return {
            ...defaultStyle,
            ...this.$frontmatter.heroImageStyle,
        };
    }
    get bgImageStyle() {
        const defaultBgImageStyle = {
            height: "350px",
            textAlign: "center",
            overflow: "hidden",
        };
        const { bgImageStyle = {} } = this.$frontmatter;
        return {
            ...defaultBgImageStyle,
            ...bgImageStyle,
        };
    }
};
BlogHero = __decorate([
    Component({ components: { MyTransition } })
], BlogHero);
export default BlogHero;
