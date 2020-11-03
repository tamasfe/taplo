import { Vue } from "vue-property-decorator";
export default class PageFooter extends Vue {
    private get footerConfig();
    /** 显示页脚 */
    private get display();
    /** 页脚内容 */
    private get footerContent();
    /** 版权信息 */
    private get copyright();
}
