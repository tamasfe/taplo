import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import MyTransition from "@theme/components/MyTransition.vue";
import NavLink from "@theme/components/NavLink.vue";
import PageFooter from "@theme/components/PageFooter.vue";
import navigate from "@theme/util/navigate";
let Home = class Home extends Vue {
    get actionLinks() {
        const { action } = this.$frontmatter;
        if (Array.isArray(action))
            return action;
        return [action];
    }
    navigate(link) {
        navigate(link, this.$router, this.$route);
    }
};
Home = __decorate([
    Component({ components: { MyTransition, NavLink, PageFooter } })
], Home);
export default Home;
