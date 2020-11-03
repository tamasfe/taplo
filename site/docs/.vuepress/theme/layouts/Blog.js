import { __decorate } from "tslib";
import { Component, Mixins } from "vue-property-decorator";
import BlogInfo from "@BlogInfo";
import BlogPage from "@BlogPage";
import Common from "@theme/components/Common.vue";
import PageEncryptMixin from "@theme/util/pageEncryptMixin";
import Password from "@theme/components/Password.vue";
let Blog = class Blog extends Mixins(PageEncryptMixin) {
};
Blog = __decorate([
    Component({ components: { BlogInfo, BlogPage, Common, Password } })
], Blog);
export default Blog;
