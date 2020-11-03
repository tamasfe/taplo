import { Vue } from "vue-property-decorator";
export default class DarkmodeSwitch extends Vue {
    private darkmode;
    /** darkmode status */
    private get darkmodeConfig();
    private mounted;
    /** 设置夜间模式 */
    setDarkmode(status: "on" | "off" | "auto"): void;
    /** 切换深色模式 */
    private toggleDarkmode;
}
