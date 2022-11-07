import { defineStore } from "pinia";

export const useLayoutStore = defineStore("layout", {
    state: () => {
        return { layout: 'dev-layout' };
    },
    actions: {
        SET_LAYOUT(layout = 'dev-layout') {
            this.layout = layout;
        },
    },
    getters: {
        layout: (state) => {
            return state.layout;
        },
    },
});
