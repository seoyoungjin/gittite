import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";
import type { StatusItem } from "@/api/types";

export const useCommitStageStore = defineStore("stage", {
  state: () => ({
    staged: [] as StatusItem[],
    unstaged: [] as StatusItem[],
  }),

  getters: {
    allStagedFiles: (state) => state.staged,
    allUnstagedFiles: (state) => state.unstaged,
    stagedFileLength: (state) => state.staged.length,
  },

  actions: {
    async updateCommitStage() {
      this.staged = await git2rs.getStatus("stage");
      this.unstaged = await git2rs.getStatus("workdir");
      // alert(JSON.stringify(this.staged));
    },
  },
});
