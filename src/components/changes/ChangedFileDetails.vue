<template>
  <q-bar dense class="q-py-md bg-grey-2">
    <OctStatusIcon v-bind:status="changedStatus" size="14pt" />
    <PathLabel v-bind:path="changedPath" />

    <q-space />

    <DiffOptions />
  </q-bar>
  <q-separator color="grey-4" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import OctStatusIcon from "@/components/OctStatusIcon.vue";
import DiffOptions from "@/components/diff/DiffOptions.vue";
import PathLabel from "@/components/PathLabel.vue";

export default defineComponent({
  name: "ChangedFileDetails",

  props: {
    selection: null,
  },

  components: {
    DiffOptions,
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
