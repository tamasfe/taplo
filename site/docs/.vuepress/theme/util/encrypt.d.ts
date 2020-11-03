/** 加密状态生成 */
import { EncryptOptions } from "../types";
/**
 * 路径命中的键
 *
 * @param encryptOptions 加密配置
 * @param path 需要判断的路径
 * @param passwordConfig 当前输入的密码
 */
export declare const pathHitKeys: (encryptOptions: EncryptOptions | undefined, path: string) => string[];
/**
 * 路径加密状态
 *
 * @param encryptOptions 加密配置
 * @param path 需要判断的路径
 * @param passwordConfig 当前输入的密码
 */
export declare const pathEncryptStatus: (encryptOptions: EncryptOptions | undefined, path: string, passwordConfig: Record<string, string>) => boolean;
