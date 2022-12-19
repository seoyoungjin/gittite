import { defineStore } from "pinia";

export const useCommitStageStore = defineStore("stage", {
  state: () => ({
    staged: [],
    files: [],
  }),

  getters: {
    allStagedFiles: (state) => state.staged,
    stagedFileLength: (state) => state.staged.length,
    allFiles: (state) => state.files,
  },

  actions: {
    updateStagedFiles(staged: []) {
      this.staged = staged;
    },
  },
});
