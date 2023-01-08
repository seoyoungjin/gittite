import { defineStore } from "pinia";
import { useCommitStageStore } from "./commitStage";
import * as git2rs from "@/lib/git2rs";
import { Repository } from "@/models/repository";
import type { BranchInfo } from "@/models/branch";
import type { CommitData } from "@/models/commit";

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo: null as Repository | null,
      current_branch: "",
      all_branches: [] as BranchInfo[],
      logList: [] as CommitData[],
    };
  },

  getters: {
    currentRepository: (state) => state.repo,
    repositoryName: (state) => {
      return state.repo ? state.repo.repoName : "";
    },
    repositoryPath: (state) => {
      return state.repo ? state.repo.repoPath : "";
    },
    currentBranch: (state) => state.current_branch,
    allBranches: (state) => state.all_branches,
    commitLogs: (state) => state.logList,
  },

  actions: {
    async setRepository(path: string) {
      const repo_path = await git2rs.setRepository(path);
      if (repo_path) {
        await this.loadRepositoryInfo();
      }
      return repo_path;
    },

    async loadRepositoryInfo() {
      const repo_info = await git2rs.get_repo_info();
      this.repo = new Repository(repo_info);

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
