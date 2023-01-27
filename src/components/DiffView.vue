<template>
  <q-scroll-area class="fit">
    <div v-html="prettyHtml" class="q-pa-xs" />
  </q-scroll-area>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import * as git2rs from "@/lib/git2rs";
import * as Diff2Html from "diff2html";
import "diff2html/bundles/css/diff2html.min.css";

export default defineComponent({
  name: "DiffView",
  props: {
    selection: null,
    selectedFile: null,
  },

  data() {
    return {
      diffs: "",
    };
  },

  computed: {
    prettyHtml() {
      return Diff2Html.html(this.diffs, {
        // drawFileList: "commit_id" in this.selection,
        drawFileList: false,
        matching: "none",
        outputFormat: "line-by-line",
      });
    },
  },

  methods: {
    getDiff: async function () {
      let current = this.selection;
      if (!current) this.diffs = "";
      if ("path" in current) {
        this.diffs = await git2rs.getDiff(current.path, "status" in current);
      } else if ("commit_id" in current) {
        this.diffs = await git2rs.getDiffCommit(
          current.commit_id,
          this.selectedFile
        );
      }
    },
  },

  watch: {
    selection: "getDiff",
    selectedFile: "getDiff",
  },
});
</script>

<style>
.d2h-file-header {
  display: none;
}
</style>
