import { Vue } from "vue-property-decorator";
export default class TagList extends Vue {
    /** 标签列表 */
    private get tagList();
    /** 是否激活 */
    private isActive;
    /** 点击标签导航 */
    private clickTag;
}
