import { Vue } from "vue-property-decorator";
export default class ThemeOptions extends Vue {
    private themeColor;
    private isDarkmode;
    private get text();
    private get themeColorEnabled();
    private get switchEnabled();
    private mounted;
    /** 设置主题 */
    private setTheme;
}
