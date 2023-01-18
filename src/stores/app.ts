// stores runtime data

import { defineStore } from "pinia";
import type { StatusItem } from "@/models/status";
import * as git2rs from "@/lib/git2rs";

export const useAppStore = defineStore("gittite", {
  state: () => ({
    props: {
      cwd: "",
      modal: false,
    },
    initialized: false,
    discard_item: null as StatusItem | null,
  }),
  getters: {
    CWD: (state) => state.props.cwd,
    Modal: (state) => state.props.modal,
    DiscardItem: (state) => state.discard_item,
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

    setDiscardItem(item: StatusItem | null) {
      this.discard_item = item;
    },
  },
});
