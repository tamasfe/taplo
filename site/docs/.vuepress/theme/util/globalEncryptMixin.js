import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import { compareSync } from "bcryptjs";
let GlobalEncryptMixin = class GlobalEncryptMixin extends Vue {
    constructor() {
        super(...arguments);
        /** 全局密码 */
        this.globalPassword = "";
    }
    /** 加密配置 */
    get encryptOptions() {
        return this.$themeConfig.encrypt || {};
    }
    /** 全局加密状态 */
    get globalEncrypted() {
        if (this.encryptOptions.status === "global" && this.encryptOptions.global) {
            const { global } = this.encryptOptions;
            /** 全局密码 */
            const globalPasswords = typeof global === "string" ? [global] : global;
            /** 全局密码匹配结果 */
            const result = globalPasswords.filter((globalPassword) => compareSync(this.globalPassword, globalPassword));
            return result.length === 0;
        }
        return false;
    }
    mounted() {
        const globalPassword = localStorage.getItem("globalPassword");
        if (globalPassword)
            this.globalPassword = globalPassword;
    }
    globalPasswordCheck(globalPassword) {
        const { global } = this.encryptOptions;
        /** 全局密码 */
        const globalPasswords = typeof global === "string" ? [global] : global;
        /** 全局密码匹配结果 */
        const result = globalPasswords.filter((password) => compareSync(globalPassword, password));
        if (result.length !== 0) {
            this.globalPassword = globalPassword;
            localStorage.setItem("globalPassword", globalPassword);
        }
    }
};
GlobalEncryptMixin = __decorate([
    Component
], GlobalEncryptMixin);
export default GlobalEncryptMixin;
