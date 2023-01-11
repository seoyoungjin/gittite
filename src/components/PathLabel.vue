<template>
  <!--
<div class="ellipsis"> {{ path }} </div>
-->
  <span class="row inline no-wrap">
    <div class="text-grey-7 ellipsis">
      {{ filePath(path) }}
    </div>
    <div>
      {{ fileName(path) }}
    </div>
    <q-tooltip :class="className">
      {{ path }}
    </q-tooltip>
  </span>
</template>

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
    className: {
      type: String,
      default: "bg-black text-body2",
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
