import { defineStore } from "pinia";
import * as git2rs from "@/api/git2rs";

export const usePropStore = defineStore("props", {
  state: () => ({
    props: {
      cwd: "",
      modal: false,
    },
    initialized: false,
  }),
  getters: {
    CWD: (state) => state.props.cwd,
    Modal: (state) => state.props.modal,
  },
  actions: {
    async initStore() {
      if (this.initialized) {
        return;
      }
      this.initialized = true;
      this.props.cwd = await git2rs.get_prop("CWD");
    },
    setPropModal(value: boolean) {
      this.props.modal = value;
      git2rs.set_prop("modal", "" + value);
    },
  },
});
