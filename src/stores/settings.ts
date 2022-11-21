import { defineStore } from "pinia";

export const useLayoutStore = defineStore("layout", {
  state: () => ({
    layout: "app-layout",
  }),
  getters: {
    getLayout: (state) => state.layout,
  },
  actions: {
    setLayout(layout = "app-layout") {
      this.layout = layout;
    },
  },
});
