<template>
  <q-bar class="q-pa-md bg-grey-2">
    <PathLabel v-bind:path="changedPath" />
    <q-space />
    <OctStatusIcon v-bind:status="changedStatus" size="14pt" />
  </q-bar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import OctStatusIcon from "@/components/OctStatusIcon.vue";
import PathLabel from "@/components/PathLabel.vue";

export default defineComponent({
  name: "ChangedFileDetails",

  props: {
    selection: null,
  },

  components: {
    OctStatusIcon,
    PathLabel,
  },

  computed: {
    changedPath() {
      return this.selection ? this.selection.path : "";
    },
    changedStatus() {
      if (!this.selection) return "";
      else if ("wtree" in this.selection) return this.selection.wtree;
      else return this.selection.status;
    },
  },
});
</script>
