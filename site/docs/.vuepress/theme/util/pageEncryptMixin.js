import { __decorate } from "tslib";
import { Component, Vue } from "vue-property-decorator";
import { compareSync } from "bcryptjs";
let PageEncryptMixin = class PageEncryptMixin extends Vue {
    constructor() {
        super(...arguments);
        this.encryptConfig = {};
    }
    /** 加密配置 */
    get encryptOptions() {
        return this.$themeConfig.encrypt || {};
    }
    /** 当前路径命中的键值 */
    get currentPathHitKeys() {
        if (this.encryptOptions && typeof this.encryptOptions.config === "object") {
            /** 配置键名 */
            const keys = Object.keys(this.encryptOptions.config);
            /** 命中键名 */
            const hitKeys = keys
                .filter((key) => this.$route.path.startsWith(key))
                .sort((a, b) => b.length - a.length);
            return hitKeys;
        }
        return [];
    }
    /** 路径是否加密 */
    get currentPathEncrypted() {
        if (this.currentPathHitKeys.length !== 0) {
            /** 配置项 */
            const { config } = this.encryptOptions;
            /** 正确键值 */
            const correctKeys = this.currentPathHitKeys.filter((key) => {
                const keyConfig = config[key];
                /** 命中的密码 */
                const hitPasswords = typeof keyConfig === "string" ? [keyConfig] : keyConfig;
                /** 比较结果 */
                const result = hitPasswords.filter((encryptPassword) => compareSync(this.encryptConfig[key], encryptPassword));
                return result.length !== 0;
            });
            return correctKeys.length === 0;
        }
        return false;
    }
    /** 设置密码 */
    setPassword(password) {
        const { config } = this.$themeConfig.encrypt;
        for (const hitKey of this.currentPathHitKeys) {
            /** 命中密码配置 */
            const hitPassword = config[hitKey];
            /** 命中密码列表 */
            const hitPasswordList = typeof hitPassword === "string" ? [hitPassword] : hitPassword;
            /** 比较结果 */
            const result = hitPasswordList.filter((encryptPassword) => compareSync(password, encryptPassword));
            // 出现匹配
            if (result.length !== 0) {
                this.$set(this.encryptConfig, hitKey, password);
                localStorage.setItem("encryptConfig", JSON.stringify(this.encryptConfig));
                break;
            }
        }
    }
    mounted() {
        const passwordConfig = localStorage.getItem("encryptConfig");
        if (passwordConfig)
            this.encryptConfig = JSON.parse(passwordConfig);
    }
};
PageEncryptMixin = __decorate([
    Component
], PageEncryptMixin);
export default PageEncryptMixin;
