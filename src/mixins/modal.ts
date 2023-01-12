import { defineComponent } from "vue";
import { mapActions } from "pinia";
import { useAppStore } from "@/stores/app";

export default defineComponent({
  methods: {
    ...mapActions(useAppStore, ["setPropModal"]),

    setModal() {
      this.setPropModal(true);
    },
    unsetModal() {
      this.setPropModal(false);
    },
  },
});
