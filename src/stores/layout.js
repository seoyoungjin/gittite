import { defineStore } from 'pinia'

export const useLayoutStore = defineStore('layout', {
  state: () => ({
    layout: "dev-layout",
  }),
  getters: {
    getLayout: (state) => state.layout,
  },
  actions: {
    setLayout(layout='dev-layout') {
      this.layout = layout;
    },
  },
})
