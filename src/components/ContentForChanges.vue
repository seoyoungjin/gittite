<template>
  <div style="height: 100vh">
    <DiffView :selection="selection" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import DiffView from "@/components/DiffView.vue";

export default defineComponent({
  name: "ContentForChanges",

  props: {
    selection: null,
  },

  components: {
    DiffView,
  },
});
</script>
