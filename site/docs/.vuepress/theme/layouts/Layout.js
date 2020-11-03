import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import BlogInfo from "@BlogInfo";
import BlogPage from "@BlogPage";
import Common from "@theme/components/Common.vue";
import Home from "@theme/components/Home.vue";
import Page from "@theme/components/Page.vue";
let Layout = class Layout extends Vue {
};
Layout = __decorate([
    Component({ components: { BlogInfo, BlogPage, Common, Home, Page } })
], Layout);
export default Layout;
