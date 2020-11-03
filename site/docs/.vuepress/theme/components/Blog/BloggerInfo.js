import { __decorate } from "tslib";
import { Component, Mixins } from "vue-property-decorator";
import { i18n } from "@mr-hope/vuepress-shared-utils";
import { ArticleMixin, TimelineMixin } from "@theme/util/articleMixin";
import MediaLinks from "@theme/components/MediaLinks.vue";
import navigate from "@theme/util/navigate";
let BloggerInfo = class BloggerInfo extends Mixins(ArticleMixin, TimelineMixin) {
    get blogConfig() {
        return this.$themeConfig.blog || {};
    }
    get bloggerName() {
        return (this.blogConfig.name || this.$themeConfig.author || this.$site.title || "");
    }
    get bloggerAvatar() {
        return this.blogConfig.avatar || this.$themeConfig.logo || "";
    }
    get hasIntro() {
        return Boolean(this.blogConfig.intro);
    }
    get i18n() {
        return this.$themeLocaleConfig.blog || i18n.getDefaultLocale().blog;
    }
    navigate(url) {
        navigate(url, this.$router, this.$route);
    }
    jumpIntro() {
        if (this.hasIntro)
            navigate(this.blogConfig.intro, this.$router, this.$route);
    }
};
BloggerInfo = __decorate([
    Component({ components: { MediaLinks } })
], BloggerInfo);
export default BloggerInfo;
