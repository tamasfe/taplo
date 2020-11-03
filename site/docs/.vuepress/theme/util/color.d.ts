/** 颜色 */
export default class Color {
    type: "hex" | "rgb";
    red: number;
    green: number;
    blue: number;
    alpha: number;
    constructor(type: "hex" | "rgb", red: number, green: number, blue: number, alpha?: number);
    /** 从 HEX 中生成 */
    static fromHex(color: string): Color;
    /** 从 RGB 或 RGBA 中生成 */
    static fromRGB(color: string): Color;
    /** 获取颜色 */
    static getColor(colorString: string): Color;
    toString(): string;
    adjust(item: "red" | "green" | "blue" | "alpha", amount: number): void;
    /** 加深颜色 */
    darken(amount: number): Color;
    /** 变浅颜色 */
    lighten(amount: number): Color;
}
