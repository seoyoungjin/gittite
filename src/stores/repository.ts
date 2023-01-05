import { defineStore } from "pinia";
import { useCommitStageStore } from "./commitStage";
import * as git2rs from "@/api/git2rs";
import type { BranchInfo, CommitData } from "@/api/types";

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo_path: "",
      current_branch: "",
      all_branches: [] as BranchInfo[],
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
    allBranches: (state) => state.all_branches,
    commitLogs: (state) => state.logList,
  },

  actions: {
    async setRepository(path: string) {
      const repo_path = await git2rs.setRepository(path);
      if (!repo_path) {
        return;
      }
      await this.loadRepositoryInfo();
      return repo_path;
    },

    async loadRepositoryInfo() {
      this.repo_path = await git2rs.workdir().catch(() => {
        return "";
      });
      if (!this.repo_path) return;

      const stageStore = useCommitStageStore();
      await stageStore.updateCommitStage();

      // TODO - misc repo data
      // remoteOriginUrl
      // last commit time
      // ahead/behind

      this.logList = await git2rs.getCommits().catch(() => {
        return [] as CommitData[];
      });

      await this.loadAllBranches();
    },

    async loadAllBranches() {
      this.current_branch = await git2rs.getBranchName().catch(() => {
        return "master";
      });
      this.all_branches = await git2rs.branchesInfo(true).catch(() => {
        return [] as BranchInfo[];
      });
    },

    getBranchInfo(name: string): BranchInfo {
      for (const branch of this.all_branches) {
        if (branch.name === name) {
          // alert(JSON.stringify(branch, null, 4));
          return branch;
        }
      }
      throw "Can not find branch information for: " + name;
    },
  },
});
