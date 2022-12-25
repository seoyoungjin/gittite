import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo_path: "",
      current_branch: "",
      all_branches: [] as string[],
    };
  },
  getters: {
    // directory name
    repositoryName: (state) => {
      return state.repo_path.split("/").reverse()[0];
    },
    repositoryPath: (state) => state.repo_path,
    currentBranch: (state) => state.current_branch,
  },
  actions: {
    async setRepository(path: string) {
      const repo_path = await git2rs.setRepository(path).catch((e) => {
        throw e;
      });
      if (!repo_path) {
        return;
      }
      this.repo_path = repo_path;
      // TODO - misc repo data
      // remoteOriginUrl
      // last commit time
      // ahead/behind
      this.current_branch = await git2rs.getBranchName().catch((err) => {
          this.current_branch = "master";
          console.log(err);
      });
      return repo_path;
    },
  },
});
