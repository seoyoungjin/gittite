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
    hasChanges: (state) => state.staged.length || state.unstaged.length,
  },

  actions: {
    async updateCommitStage() {
      this.staged = await git2rs.getStatus("stage").catch(() => {
        return [] as StatusItem[];
      });
      this.unstaged = await git2rs.getStatus("workdir").catch(() => {
        return [] as StatusItem[];
      });

      if (!this.currentItem) {
        if (this.unstaged.length) {
          this.currentItem = this.unstaged[0];
        } else if (this.staged.length) {
          this.currentItem = this.staged[0];
        }
      }
    },

    setCurrentItem(current: StatusItem) {
      this.currentItem = current;
    },

    async discardAllChanges() {
      this.staged.forEach(async (k) => {
        await git2rs.resetStage(k.path);
      });
      // load unstaged after resetStage()
      const unstaged = await git2rs.getStatus("workdir").catch(() => {
        return [] as StatusItem[];
      });
      unstaged.forEach(async (k) => {
        await git2rs.resetWorkdir(k.path);
      });
      await this.updateCommitStage();
    },
  },
});
