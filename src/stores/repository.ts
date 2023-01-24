import { defineStore } from "pinia";
import { useCommitStageStore } from "./commit-stage";
import { useHistoryStore } from "./history";
import * as git2rs from "@/lib/git2rs";
import { Repository } from "@/models/repository";
import type { BranchInfo } from "@/models/branch";
import type { Tag } from "@/models/tag";

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo: null as Repository | null,
      current_branch: "",
      all_branches: [] as BranchInfo[],
      branch_to_switch: "",
      all_tags: new Map<String, Tag[]>(),
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
    branchToSwitch: (state) => state.branch_to_switch,
    allBranches: (state) => state.all_branches,
    allTags: (state) => state.all_tags,
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
      const repo_info = await git2rs.getRepositoryInfo();
      this.repo = new Repository(repo_info);

      const stageStore = useCommitStageStore();
      await stageStore.updateCommitStage();

      await this.loadAllBranches();
      await this.loadAllTags();

      const historyStore = useHistoryStore();
      historyStore.resetHistory();
      await historyStore.loadCommitBatch("HEAD", 0);

      // TODO - misc repo data
      // remoteOriginUrl
      // last commit time
      // ahead/behind
    },

    async loadAllTags() {
      const tags = await git2rs.getTags().catch(() => {
        return [];
      });
      // list => Map
      for (const item of tags) {
        this.all_tags.set(item[0], item[1]);
      }
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
      const idx = this.all_branches.findIndex((b) => b.name === name);
      if (idx < 0) {
        throw new Error("Can not find branch information for: " + name);
      }
      return this.all_branches[idx];
    },

    setBranchToSwitch(name: string) {
      this.branch_to_switch = name;
    },
  },
});
