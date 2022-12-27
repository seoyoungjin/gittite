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

      this.loadRepositoryInfo();

      return repo_path;
    },

    loadRepositoryInfo() {
      // TODO - misc repo data
      // remoteOriginUrl
      // last commit time
      // ahead/behind
      git2rs
        .getBranchName()
        .then((res) => {
          this.current_branch = res as string;
        })
        .catch((err) => {
          console.log(err);
          this.current_branch = "master";
        });
    },
  },
});
