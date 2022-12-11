import * as git2rs from "@/api/git2rs";

export default {
  methods: {
    onDialogShow() {
      git2rs.set_prop("modal", "true");
    },
    onDialogHide() {
      git2rs.set_prop("modal", "false");
    },
  },
};
