import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import BlogInfoList from "@theme/components/Blog/BlogInfoList.vue";
import BloggerInfo from "@theme/components/Blog/BloggerInfo.vue";
import MyTransition from "@theme/components/MyTransition.vue";
let BlogInfo = class BlogInfo extends Vue {
};
BlogInfo = __decorate([
    Component({
        components: { BlogInfoList, BloggerInfo, MyTransition },
    })
], BlogInfo);
export default BlogInfo;
