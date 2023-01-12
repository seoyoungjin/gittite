import { defineStore } from "pinia";
import * as git2rs from "@/lib/git2rs";
import type { StatusItem } from "@/models/status";

export const useCommitStageStore = defineStore("stage", {
  state: () => ({
    currentItem: null as StatusItem | null,
    staged: [] as StatusItem[],
    unstaged: [] as StatusItem[],
  }),

  getters: {
    changesCurrent: (state) => state.currentItem,
    allStagedFiles: (state) => state.staged,
    allUnstagedFiles: (state) => state.unstaged,
    stagedFileLength: (state) => state.staged.length,
  },

  actions: {
    async updateCommitStage() {
      this.staged = await git2rs.getStatus("stage");
      this.unstaged = await git2rs.getStatus("workdir");
      // alert(JSON.stringify(this.staged));

      if (!this.currentItem) {
        if (this.allUnstagedFiles.length) {
          this.currentItem = this.allUnstagedFiles[0];
        } else if (this.allStagedFiles.length) {
          this.currentItem = this.allStagedFiles[0];
        }
      }
    },

    setCurrentItem(current: StatusItem) {
      this.currentItem = current;
    },
  },
});
