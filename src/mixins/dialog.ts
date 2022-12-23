// import * as git2rs from "@/api/git2rs";
import { defineComponent } from "vue";
import { mapActions } from "pinia";
import { usePropStore } from "@/stores/props";

export default defineComponent({
  methods: {
    ...mapActions(usePropStore, ["setModal"]),

    onDialogShow() {
      // git2rs.set_prop("modal", "true");
      this.setModal(true);
    },
    onDialogHide() {
      // git2rs.set_prop("modal", "false");
      this.setModal(false);
    },
  },
});
