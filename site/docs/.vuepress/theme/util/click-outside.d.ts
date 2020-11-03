import { DirectiveOptions, VNode } from "vue";
import { DirectiveBinding } from "vue/types/options";
declare type Event = TouchEvent | MouseEvent;
/** Popup HTML 事件 */
interface PopupHtmlElements extends HTMLElement {
    $vueClickOutside?: {
        callback: (event: Event) => void;
        handler: (event: Event) => void;
    };
}
/** Popup 指令函数 */
declare type PopupDirectiveFunction = (el: PopupHtmlElements, binding: DirectiveBinding, vnode: VNode, oldVnode: VNode) => void;
export declare const bind: PopupDirectiveFunction;
/** 更新命令 */
export declare const update: PopupDirectiveFunction;
/** 解绑命令 */
export declare const unbind: PopupDirectiveFunction;
declare const _default: DirectiveOptions;
export default _default;
