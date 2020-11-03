import GlobalEncryptMixin from "@theme/util/globalEncryptMixin";
declare const Common_base: import("vue-class-component/lib/declarations").VueClass<GlobalEncryptMixin>;
export default class Common extends Common_base {
    private readonly navbar;
    private readonly sidebar;
    private isSidebarOpen;
    private touchStart;
    /** 是否应该展示导航栏 */
    private get showNavbar();
    /** 是否应该展示侧边栏 */
    private get showSidebar();
    /** 侧边栏内容 */
    private get sidebarItems();
    /** 页面 Class */
    private get pageClasses();
    private get headers();
    private get showAnchor();
    protected mounted(): void;
    private toggleSidebar;
    private onTouchStart;
    private onTouchEnd;
    private getHeader;
}
export {};
