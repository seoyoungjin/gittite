import { defineStore } from "pinia";
import { useCommitStageStore } from "./commitStage";
import * as git2rs from "@/lib/git2rs";
import { Repository } from "@/models/repository";
import type { BranchInfo } from "@/models/branch";
import type { CommitData } from "@/models/commit";

const CommitBatchSize = 100;
const LoadingHistoryRequestKey = "history";

const requestsInFight = new Set<string>();

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo: null as Repository | null,
      current_branch: "",
      all_branches: [] as BranchInfo[],
      commit_list: [] as CommitData[],
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
    commitLogs: (state) => state.commit_list,
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

      // TODO - misc repo data
      // remoteOriginUrl
      // last commit time
      // ahead/behind

      this.commit_list = [];
      await this.loadCommitBatch("HEAD", 0);

      await this.loadAllBranches();
    },

    // Load a batch of commits from the repository,
    // using a given commitish object as the starting point
    async loadCommitBatch(commitish: string, skip: number): Promise<any> {
      if (requestsInFight.has(LoadingHistoryRequestKey)) {
        return null;
      }

      const requestKey = `history/compare/${commitish}/skip/${skip}`;
      if (requestsInFight.has(requestKey)) {
        return null;
      }

      requestsInFight.add(requestKey);

      const commits = await git2rs.getCommits(commitish, CommitBatchSize, skip);

      requestsInFight.delete(requestKey);
      if (!commits) {
        return null;
      }

      this.storeCommits_(commits);
      return commits.map((c) => c.commit_id);
    },

    async loadNextCommitBatch(): Promise<any> {
      return await this.loadCommitBatch("HEAD", this.commit_list.length);
    },

    storeCommits_(newCommits: CommitData[]) {
      this.commit_list = this.commit_list.concat(newCommits);
      /*
      for (cosnt commit of commits) {
        this.commitLookup.set(commit.sha, commit)
      }
      */
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
      throw new Error("Can not find branch information for: " + name);
    },
  },
});
