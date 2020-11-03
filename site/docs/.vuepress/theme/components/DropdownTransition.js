import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
let DropdownTransition = class DropdownTransition extends Vue {
    setHeight(items) {
        // explicitly set height so that it can be transitioned
        items.style.height = `${items.scrollHeight}px`;
    }
    unsetHeight(items) {
        items.style.height = "";
    }
};
DropdownTransition = __decorate([
    Component
], DropdownTransition);
export default DropdownTransition;
