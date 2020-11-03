import { Vue } from "vue-property-decorator";
export default class DropdownLink extends Vue {
    private readonly item;
    private open;
    private get dropdownAriaLabel();
    private get iconPrefix();
    private setOpen;
    handleDropdown(event: MouseEvent): void;
    private isLastItemOfArray;
    onRouteChange(): void;
}
