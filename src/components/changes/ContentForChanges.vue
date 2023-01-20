<template>
  <div style="height: 100vh">
    <ChangedFileDetails :selection="changesCurrent" />
    <DiffView :selection="changesCurrent" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import { useCommitStageStore } from "@/stores/commit-stage";
import ChangedFileDetails from "./ChangedFileDetails.vue";
import DiffView from "@/components/DiffView.vue";

export default defineComponent({
  name: "ContentForChanges",

  components: {
    ChangedFileDetails,
    DiffView,
  },

  computed: {
    ...mapState(useCommitStageStore, ["changesCurrent"]),
  },
});
</script>
