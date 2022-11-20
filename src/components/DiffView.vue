<template>
  <div v-html="prettyHtml" />
</template>

<script lang="ts">
import * as git2rs from "@/api/git2rs";
import * as Diff2Html from "diff2html";
import "diff2html/bundles/css/diff2html.min.css";

export default {
  name: "DiffView",
  props: {
    curSelected: Object,
  },

  data() {
    return {
      diffs: "",
    };
  },

  computed: {
    prettyHtml() {
      return Diff2Html.html(this.diffs, {
        drawFileList: false,
        matching: "lines",
        outputFormat: "line-by-line",
        highlight: true,
      });
    },
  },

  methods: {
    getDiff: async function () {
      // alert(JSON.stringify(this.curSelected));
      let current = this.curSelected;
      if (current == null || !("path" in current)) return;
      this.diffs = await git2rs.getDiff(current.path, "stage" in current);
    },
  },

  watch: {
    curSelected: "getDiff",
  },
};
</script>
