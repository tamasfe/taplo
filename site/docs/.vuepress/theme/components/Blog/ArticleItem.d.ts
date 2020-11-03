import { Vue } from "vue-property-decorator";
export default class ArticleItem extends Vue {
    private readonly article;
    /** 文章是否加密 */
    private get isEncrypted();
}
