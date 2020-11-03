import { Vue } from "vue-property-decorator";
export default class NavLink extends Vue {
    private readonly item;
    private get link();
    private get iconPrefix();
    private get active();
    private get isNonHttpURI();
    private get isBlankTarget();
    private get isInternal();
    private get target();
    private get rel();
    private focusoutAction;
}
