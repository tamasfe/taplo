"use strict";
exports.__esModule = true;
/** 颜色 */
var Color = /** @class */ (function () {
    function Color(type, red, green, blue, alpha) {
        if (alpha === void 0) { alpha = 1; }
        this.type = type;
        this.red = red;
        this.green = green;
        this.blue = blue;
        this.alpha = alpha;
    }
    /** 从 HEX 中生成 */
    Color.fromHex = function (color) {
        var parseHex = function (colorString) { return parseInt(colorString, 16); };
        var parseAlpha = function (colorString, total) {
            return Math.round((parseHex(colorString) * 100) / total) / 100;
        };
        if (color.length === 4)
            return new Color("hex", parseHex(color[1]) * 17, parseHex(color[2]) * 17, parseHex(color[3]) * 17);
        if (color.length === 5)
            return new Color("hex", parseHex(color[1]) * 17, parseHex(color[2]) * 17, parseHex(color[3]) * 17, parseAlpha(color[4], 15));
        if (color.length === 7)
            return new Color("hex", parseHex(color.substring(1, 3)), parseHex(color.substring(3, 5)), parseHex(color.substring(5, 7)));
        return new Color("hex", parseHex(color.substring(1, 3)), parseHex(color.substring(3, 5)), parseHex(color.substring(5, 7)), parseAlpha(color.substring(7, 9), 255));
    };
    /** 从 RGB 或 RGBA 中生成 */
    Color.fromRGB = function (color) {
        // eslint-disable-next-line @typescript-eslint/naming-convention
        var RGBAPattern = /rgba\((.+)?,(.+)?,(.+)?,(.+)?\)/u;
        // eslint-disable-next-line @typescript-eslint/naming-convention
        var RGBPattern = /rgb\((.+)?,(.+)?,(.+)?\)/u;
        var fromRGB = function (colorString) {
            return colorString.includes("%")
                ? (Number(colorString.trim().substring(0, colorString.trim().length - 1)) /
                    100) *
                    256 -
                    1
                : Number(colorString.trim());
        };
        var rgbaResult = RGBAPattern.exec(color);
        if (rgbaResult)
            return new Color("rgb", fromRGB(rgbaResult[1]), fromRGB(rgbaResult[2]), fromRGB(rgbaResult[3]), Number(rgbaResult[4] || 1));
        var rgbResult = RGBPattern.exec(color);
        if (rgbResult)
            return new Color("rgb", fromRGB(rgbResult[1]), fromRGB(rgbResult[2]), fromRGB(rgbResult[3]));
        throw new Error("Can not handle color: " + color);
    };
    /** 获取颜色 */
    Color.getColor = function (colorString) {
        if (colorString.startsWith("#"))
            return this.fromHex(colorString);
        return this.fromRGB(colorString);
    };
    Color.prototype.toString = function () {
        if (this.type === "hex" && this.alpha === 1) {
            var toHex_1 = function (color) {
                return color < 10
                    ? color.toString()
                    : color === 10
                        ? "a"
                        : color === 11
                            ? "b"
                            : color === 12
                                ? "c"
                                : color === 13
                                    ? "d"
                                    : color === 14
                                        ? "e"
                                        : "f";
            };
            if (this.red % 17 === 0 && this.green % 17 === 0 && this.blue % 17 === 0)
                return "#" + toHex_1(this.red / 17) + toHex_1(this.green / 17) + toHex_1(this.blue / 17);
            var getHex = function (color) {
                return toHex_1((color - (color % 16)) / 16) + toHex_1(color % 16);
            };
            return "#" + getHex(this.red) + getHex(this.green) + getHex(this.blue);
        }
        return this.alpha === 1
            ? "rgb(" + this.red + "," + this.green + "," + this.blue + ")"
            : "rgba(" + this.red + "," + this.green + "," + this.blue + "," + this.alpha + ")";
    };
    Color.prototype.adjust = function (item, amount) {
        var result = Math.round(this[item] * amount);
        if (item === "alpha")
            this.alpha = result < 0 ? 0 : result > 1 ? 1 : result;
        else
            this[item] = result < 0 ? 0 : result > 255 ? 255 : result;
    };
    /** 加深颜色 */
    Color.prototype.darken = function (amount) {
        this.adjust("red", 1 - amount);
        this.adjust("green", 1 - amount);
        this.adjust("blue", 1 - amount);
        return this;
    };
    /** 变浅颜色 */
    Color.prototype.lighten = function (amount) {
        this.adjust("red", 1 + amount);
        this.adjust("green", 1 + amount);
        this.adjust("blue", 1 + amount);
        return this;
    };
    return Color;
}());
exports["default"] = Color;
