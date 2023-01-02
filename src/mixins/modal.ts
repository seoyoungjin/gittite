import { defineComponent } from "vue";
import { mapActions } from "pinia";
import { usePropStore } from "@/stores/props";

export default defineComponent({
  methods: {
    ...mapActions(usePropStore, ["setPropModal"]),

    setModal() {
      this.setPropModal(true);
    },
    unsetModal() {
      this.setPropModal(false);
    },
  },
});
