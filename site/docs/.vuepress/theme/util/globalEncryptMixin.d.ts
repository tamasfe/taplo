import { Vue } from "vue-property-decorator";
import { EncryptOptions } from "../types";
export default class GlobalEncryptMixin extends Vue {
    /** 全局密码 */
    protected globalPassword: string;
    /** 加密配置 */
    protected get encryptOptions(): EncryptOptions;
    /** 全局加密状态 */
    protected get globalEncrypted(): boolean;
    protected mounted(): void;
    protected globalPasswordCheck(globalPassword: string): void;
}
