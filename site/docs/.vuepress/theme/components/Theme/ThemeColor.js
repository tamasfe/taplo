import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import ClickOutside from "@theme/util/click-outside";
import ThemeOptions from "@theme/components/Theme/ThemeOptions.vue";
let ThemeColor = class ThemeColor extends Vue {
    constructor() {
        super(...arguments);
        this.showMenu = false;
    }
    clickOutside() {
        this.showMenu = false;
    }
};
ThemeColor = __decorate([
    Component({
        directives: { "click-outside": ClickOutside },
        components: { ThemeOptions },
    })
], ThemeColor);
export default ThemeColor;
