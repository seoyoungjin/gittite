<template>
  <div style="height: 100vh">
    <DiffView :selection="changesCurrent" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import { useCommitStageStore } from "@/stores/commit-stage";
import DiffView from "@/components/DiffView.vue";

export default defineComponent({
  name: "ContentForChanges",

  components: {
    DiffView,
  },

  computed: {
    ...mapState(useCommitStageStore, ["changesCurrent"]),
  },
});
</script>
