import { Vue } from "vue-property-decorator";
export default class SidebarGroup extends Vue {
    private readonly item;
    private readonly open;
    private readonly depth;
    private isActive;
    private getIcon;
    private beforeCreate;
}
