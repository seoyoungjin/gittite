import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

const directory = await git2rs.get_prop("CWD");

export const usePropStore = defineStore("props", {
  state: () => ({
    props: {
      cwd: directory,
      modal: false,
    },
  }),
  getters: {
    CWD: (state) => state.props.cwd,
    Modal: (state) => state.props.modal,
  },
  actions: {
    setCwd(value: string) {
      this.props.cwd = value;
      git2rs.set_prop("cwd", value);
    },
    setModal(value: boolean) {
      this.props.modal = value;
      git2rs.set_prop("modal", "" + value);
    },
  },
});
