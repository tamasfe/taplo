import { Vue } from "vue-property-decorator";
export default class SidebarLinks extends Vue {
    private readonly items;
    private readonly depth;
    private openGroupIndex;
    private refreshIndex;
    private toggleGroup;
    private isActive;
    private created;
    onRouteUpdate(): void;
}
