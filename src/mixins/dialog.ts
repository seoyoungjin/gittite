import * as git2rs from "@/api/git2rs";
// import { mapActions } from "pinia";
// import { usePropStore } from "@/stores/prop";

export default {
  methods: {
    // ...mapActions(usePropStore, ["setModal"]),

    onDialogShow() {
      git2rs.set_prop("modal", "true");
      // this.setModal(true);
    },
    onDialogHide() {
      git2rs.set_prop("modal", "false");
      // this.setModal(false);
    },
  },
};
