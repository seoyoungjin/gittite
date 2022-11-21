import { defineStore, acceptHMRUpdate } from "pinia";

export const useCommitStageStore = defineStore("stage", {
  state: () => ({
    activeBranch: "",
    staged: [],
    files: [],
  }),

  getters: {
    allStagedFiles: (state) => state.staged,
    stagedFileLength: (state) => state.staged.length,
    allFiles: (state) => state.files,
  },

  actions: {
    updateActiveBranch(branch) {
      this.activeBranch = branch;
    },
    updateStagedFiles(staged) {
      this.staged = staged;
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useCommitStageStore, import.meta.hot));
}
