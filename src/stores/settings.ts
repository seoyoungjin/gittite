import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

export const useSettingsStore = defineStore("settings", {
  state: () => ({
    settings: {
      profile: {
        name: "",
        email: "",
        image_url: "",
      },
      all_repository: [] as string[],
    },
  }),
  getters: {
    getSettings: (state) => state.settings,
    getProfile: (state) => state.settings.profile,
    allRepository: (state) => state.settings.all_repository,
  },
  actions: {
    async loadSettings() {
      this.settings = await git2rs.loadSettings();
      // alert(JSON.stringify(this.settings));
    },
    saveSettings() {
      git2rs.saveSettings(this.settings);
    },
    addRepository(path: string) {
      this.settings.all_repository.push(path);
      //alert(this.settings.all_repository);
      git2rs.saveSettings(this.settings);
    },
    setProfile(name: string, email: string, image: string) {
      this.settings.profile.name = name;
      this.settings.profile.email = email;
      this.settings.profile.image_url = image;
      // alert(JSON.stringify(this.settings.profile));
    },
  },
});
