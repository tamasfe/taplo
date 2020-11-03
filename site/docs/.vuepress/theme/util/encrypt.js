/** 加密状态生成 */
/**
 * 路径命中的键
 *
 * @param encryptOptions 加密配置
 * @param path 需要判断的路径
 * @param passwordConfig 当前输入的密码
 */
export const pathHitKeys = (encryptOptions, path) => {
    if (encryptOptions && typeof encryptOptions.config === "object") {
        /** 配置键名 */
        const keys = Object.keys(encryptOptions.config);
        /** 命中键名 */
        const hitKeys = keys
            .filter((key) => path.startsWith(key))
            .sort((a, b) => b.length - a.length);
        return hitKeys;
    }
    return [];
};
/**
 * 路径加密状态
 *
 * @param encryptOptions 加密配置
 * @param path 需要判断的路径
 * @param passwordConfig 当前输入的密码
 */
export const pathEncryptStatus = (encryptOptions, path, passwordConfig) => {
    /** 命中键名 */
    const hitKeys = pathHitKeys(encryptOptions, path);
    if (hitKeys.length !== 0) {
        /** 配置项 */
        const { config } = encryptOptions;
        /** 正确键值 */
        const correctKeys = hitKeys.filter((key) => {
            const keyConfig = config[key];
            /** 命中的密码 */
            const hitPasswords = typeof keyConfig === "string" ? [keyConfig] : keyConfig;
            /** 比较结果 */
            const result = hitPasswords.filter((password) => passwordConfig[key] === password);
            return result.length !== 0;
        });
        return correctKeys.length === 0;
    }
    return false;
};
