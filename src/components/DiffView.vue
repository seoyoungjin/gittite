<template>
  <div v-html="prettyHtml" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import * as git2rs from "@/lib/git2rs";
import * as Diff2Html from "diff2html";
import "diff2html/bundles/css/diff2html.min.css";

export default defineComponent({
  name: "DiffView",
  props: {
    curSelected: null,
  },

  data() {
    return {
      diffs: "",
    };
  },

  computed: {
    prettyHtml() {
      return Diff2Html.html(this.diffs, {
        drawFileList: "commit_id" in this.curSelected,
        matching: "none",
        outputFormat: "line-by-line",
      });
    },
  },

  methods: {
    getDiff: async function () {
      // alert(JSON.stringify(this.curSelected));
      let current = this.curSelected;
      if ("path" in current) {
        this.diffs = await git2rs.getDiff(current.path, "stage" in current);
      } else if ("commit_id" in current) {
        this.diffs = await git2rs.getDiffCommit(current.commit_id, null);
      }
    },
  },

  watch: {
    curSelected: "getDiff",
  },
});
</script>
