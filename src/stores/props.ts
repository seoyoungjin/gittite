import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

const directory = await git2rs.get_prop("CWD");

export const usePropsStore = defineStore("props", {
  state: () => ({
    props: {
      cwd: directory
    }
  }),
  getters: {
    CWD: (state) => state.props.cwd,
  },
  actions: {
    setCwd(value: string) {
      // TODO - tauri
      this.props.cwd = value;
    },
  },
});
