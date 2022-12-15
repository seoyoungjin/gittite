import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

export const useSettingsStore = defineStore("settings", {
  state: () => ({
    settings: {
      repo: "",
      all_repository: [],
    },
  }),
  getters: {
    getSettings: (state) => state.settings,
  },
  actions: {
    setSettings(settings) {
      this.settings = settings;
    },
    addRepository(path: string) {
      this.settings.all_repository.push(path);
      //alert(this.settings.all_repository);
      git2rs.saveSettings(this.settings);
    },
  },
});
