import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import ArticleList from "@theme/components/Blog/ArticleList.vue";
import { capitalize } from "@mr-hope/vuepress-shared-utils";
import navigate from "@theme/util/navigate";
let CategoryList = class CategoryList extends Vue {
    constructor() {
        super(...arguments);
        /** 大写首字母 */
        this.capitalize = (name) => capitalize(name);
    }
    /** 点击分类的导航 */
    clickCategory(path) {
        navigate(path, this.$router, this.$route);
    }
};
CategoryList = __decorate([
    Component({ components: { ArticleList } })
], CategoryList);
export default CategoryList;
