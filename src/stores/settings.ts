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
    allRepository: (state) => state.settings.all_repository,
  },
  actions: {
    async loadSettings() {
      this.settings = await git2rs.loadSettings();
      // alert(JSON.stringify(this.settings));
    },
    addRepository(path: string) {
      this.settings.all_repository.push(path);
      //alert(this.settings.all_repository);
      git2rs.saveSettings(this.settings);
    },
  },
});
