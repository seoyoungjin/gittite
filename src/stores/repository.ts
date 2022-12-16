import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

const directory = await git2rs.get_prop("CWD");

export const useRepositoryStore = defineStore("repository", {
  state: () => {
    return {
      repo_path: directory,
      current_branch: "",
      all_branches: [],
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
    setRepository(path: string) {
      // TODO set_repository
      // error handle
      git2rs.getRepository(path);
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
