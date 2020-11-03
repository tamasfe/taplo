import { __decorate } from "tslib";
import { Component, Prop, Vue } from "vue-property-decorator";
let MyTransition = class MyTransition extends Vue {
    setStyle(items) {
        items.style.transition = `transform 0.1s ease-in-out 0s, opacity 0.1s ease-in-out 0s`;
        items.style.opacity = "0";
    }
    unsetStyle(items) {
        items.style.opacity = "1";
    }
};
__decorate([
    Prop({ type: Number, default: 0 })
], MyTransition.prototype, "delay", void 0);
__decorate([
    Prop({ type: Number, default: 0.25 })
], MyTransition.prototype, "duration", void 0);
MyTransition = __decorate([
    Component
], MyTransition);
export default MyTransition;
