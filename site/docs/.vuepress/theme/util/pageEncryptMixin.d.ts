import { Vue } from "vue-property-decorator";
import { EncryptOptions } from "../types";
export default class PageEncryptMixin extends Vue {
    protected encryptConfig: Record<string, string>;
    /** 加密配置 */
    protected get encryptOptions(): EncryptOptions;
    /** 当前路径命中的键值 */
    protected get currentPathHitKeys(): string[];
    /** 路径是否加密 */
    protected get currentPathEncrypted(): boolean;
    /** 设置密码 */
    protected setPassword(password: string): void;
    protected mounted(): void;
}
