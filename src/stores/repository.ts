import { defineStore } from "pinia";
import { useCommitStageStore } from "./commitStage";
import * as git2rs from "@/api/git2rs";
import type { CommitData } from "@/api/types";

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo_path: "",
      current_branch: "",
      all_branches: [] as string[],
      logList: [] as CommitData[],
    };
  },
  getters: {
    repositoryName: (state) => {
      const arr = state.repo_path.split("/").reverse().filter(Boolean);
      if (arr.length == 0) return "";
      return arr[0];
    },
    repositoryPath: (state) => state.repo_path,
    currentBranch: (state) => state.current_branch,
    commitLogs: (state) => state.logList,
  },
  actions: {
    async setRepository(path: string) {
      const repo_path = await git2rs.setRepository(path);
      if (!repo_path) {
        return;
      }
      this.loadRepositoryInfo();
      return repo_path;
    },

    async loadRepositoryInfo() {
      this.repo_path = await git2rs.workdir().catch(() => {
        return "";
      });
      if (!this.repo_path)
        return;

      const stageStore = useCommitStageStore();
      stageStore.updateCommitStage();

      // TODO - misc repo data
      // remoteOriginUrl
      // last commit time
      // ahead/behind
      this.current_branch = await git2rs.getBranchName().catch(() => {
        return "master";
      });

      this.logList = await git2rs.getCommits();
    },
  },
});
