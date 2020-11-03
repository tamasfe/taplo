import { Context, PluginOptionAPI } from "@mr-hope/vuepress-types";
import { ResolvedHopeThemeConfig } from "./types";
interface ThemeOptionAPI extends PluginOptionAPI {
    extend?: string;
}
declare const themeAPI: {
    (themeConfig: ResolvedHopeThemeConfig, ctx: Context): ThemeOptionAPI;
    config: (config: import("./types").HopeVuepressConfig) => import("./types").ResolvedHopeVuepressConfig;
};
export = themeAPI;
