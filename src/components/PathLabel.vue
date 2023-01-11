<template>
  <span class="row inline no-wrap">
    <div class="text-grey-7">
      {{ filePath(path) }}
    </div>
    <div>
      {{ fileName(path) }}
    </div>
  </span>
  <!--
  <div class="ellipsis">
    {{ path }}
  </div>
--></template>

<script lang="ts">
// TODO Renamed, Copied
import { defineComponent } from "vue";

export default defineComponent({
  name: "PathLabel",

  props: {
    path: {
      type: String,
      default: "",
    },
    availableWidth: {
      type: Number,
      required: false,
    },
  },
  methods: {
    filePath(path: string) {
      if (path.lastIndexOf("/").toString() !== "-1") {
        return path.slice(0, path.lastIndexOf("/"));
      }
    },
    fileName(path: string) {
      if (path.lastIndexOf("/").toString() !== "-1") {
        return path.slice(path.lastIndexOf("/"), path.length);
      }
      return path;
    },
  },
});
</script>
