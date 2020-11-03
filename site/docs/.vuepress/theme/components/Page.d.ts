import { Vue } from "vue-property-decorator";
export default class Page extends Vue {
    private readonly sidebarItems;
    private readonly headers;
    /** 用户输入的密码 */
    private password;
    /** 是否启用评论 */
    private commentEnable;
    /** 当前页面密码 */
    private get pagePassword();
    /** 当前页面解密状态 */
    private get pageDescrypted();
}
