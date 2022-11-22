import { defineStore } from "pinia";

export const useSettingsStore = defineStore("settings", {
  state: () => ({
    settings: {
      repo: String | null;
      allRepository: [];
      repositoryData: {}
    }
  }),
  getters: {
    getSettings: (state) => state.settings,
  },
  actions: {
    setSettings(settings) {
      this.settings = settings;
    },
  },
});
