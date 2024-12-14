import "vue-virtual-scroller/dist/vue-virtual-scroller.css";
//@ts-ignore
import {RecycleScroller} from "vue-virtual-scroller";
//@ts-ignore
import {DynamicScroller} from "vue-virtual-scroller";
//@ts-ignore
import {DynamicScrollerItem} from "vue-virtual-scroller/dist/vue-virtual-scroller.esm";

export default defineNuxtPlugin((nuxtApp) => {
    nuxtApp.vueApp.component("RecycleScroller", RecycleScroller);
    nuxtApp.vueApp.component("DynamicScroller", DynamicScroller);
    nuxtApp.vueApp.component("DynamicScrollerItem", DynamicScrollerItem);
});