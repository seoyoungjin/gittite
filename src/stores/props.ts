import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

const directory = (await git2rs.get_param("cwd"))["Str"];

export const usePropsStore = defineStore("props", {
  state: () => ({
    props: {
      _cwd: directory
    }
  }),
  getters: {
    cwd: (state) => state.props._cwd,
  },
  actions: {
    setCwd(value: string) {
      // TODO - tauri
      this.props._cwd = value;
    },
  },
});
