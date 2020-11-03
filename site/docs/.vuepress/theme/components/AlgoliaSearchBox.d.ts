import { Vue } from "vue-property-decorator";
import { AlgoliaOption } from "@mr-hope/vuepress-types";
export default class AlgoliaSearchBox extends Vue {
    options: AlgoliaOption;
    private placeholder;
    onLangChange(newValue: string): void;
    onOptionsChange(newValue: AlgoliaOption): void;
    private mounted;
    private initialize;
    private update;
}
