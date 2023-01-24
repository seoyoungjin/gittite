<template>
  <div v-if="hasChanges" style="height: 100vh">
    <ChangedFileDetails :selection="changesCurrent" />
    <DiffView :selection="changesCurrent" />
  </div>
  <div v-else class="q-pa-lg">
    <NoChanges />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import { useCommitStageStore } from "@/stores/commit-stage";
import ChangedFileDetails from "./ChangedFileDetails.vue";
import NoChanges from "./NoChanges.vue";
import DiffView from "@/components/DiffView.vue";

export default defineComponent({
  name: "ContentForChanges",

  components: {
    ChangedFileDetails,
    DiffView,
    NoChanges,
  },

  computed: {
    ...mapState(useCommitStageStore, ["changesCurrent", "hasChanges"]),
  },
});
</script>
