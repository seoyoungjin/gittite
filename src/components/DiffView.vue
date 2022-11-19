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
    path: String,
  },

  data() {
    return {
      diffs: "",
    };
  },

  computed: {
    prettyHtml() {
      return Diff2Html.html(this.diffs, {
        drawFileList: true,
        matching: "lines",
        outputFormat: "line-by-line",
        highlight: true,
      });
    },
  },

  methods: {
    getDiff: async function () {
      this.diffs = await git2rs.diff();
    },
  },

  watch: {
    path: "getDiff",
  },
};
</script>
