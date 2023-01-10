<template>
  <diff-view :selection="selection" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import DiffView from "@/components/DiffView.vue";

export default defineComponent({
  name: "ContentForChanges",

  props: {
    selection: {},
  },

  components: {
    DiffView,
  },
});
</script>
