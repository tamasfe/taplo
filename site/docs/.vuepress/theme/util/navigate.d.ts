import VueRouter, { Route } from "vue-router";
/**
 * 导航
 *
 * @param url 跳转的网址
 * @param router 路由管理器
 * @param route 当前页面路由
 */
declare const navigate: (url: string, router: VueRouter, route: Route) => void;
export default navigate;
