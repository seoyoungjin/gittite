import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

const directory = await git2rs.get_prop("CWD");

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo_path: "",
      current_branch: "",
      all_branches: [],
    };
  },
  getters: {
    repositoryPath: (state) => state.repo_path,
    currentBranch: (state) => state.current_branch,
  },
  actions: {
    setRepository(path: string) {
      // TODO set_repository
      this.repo_path = path;
      git2rs
        .getBranchName()
        .then((res) => {
          this.current_branch = res;
          console.log("branch: " + res);
        })
        .catch((err) => {
          console.log(err);
          this.current_branch = "master";
        });
    },
  },
});
