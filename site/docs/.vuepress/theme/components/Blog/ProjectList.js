import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import ArticleIcon from "@mr-hope/vuepress-shared-utils/icons/ArticleIcon.vue";
import BookIcon from "@mr-hope/vuepress-shared-utils/icons/BookIcon.vue";
import LinkIcon from "@mr-hope/vuepress-shared-utils/icons/LinkIcon.vue";
import ProjectIcon from "@mr-hope/vuepress-shared-utils/icons/ProjectIcon.vue";
import navigate from "@theme/util/navigate";
let ProjectList = class ProjectList extends Vue {
    navigate(link) {
        navigate(link, this.$router, this.$route);
    }
};
ProjectList = __decorate([
    Component({
        components: { ArticleIcon, BookIcon, LinkIcon, ProjectIcon },
    })
], ProjectList);
export default ProjectList;
