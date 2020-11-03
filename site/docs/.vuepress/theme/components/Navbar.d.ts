import { Vue } from "vue-property-decorator";
export default class Navbar extends Vue {
    private linksWrapMaxWidth;
    /** Algolia 配置 */
    private get algolia();
    /** 是否使用 Algolia 搜索 */
    private get isAlgoliaSearch();
    private mounted;
}
