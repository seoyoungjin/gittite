<template>
  <div class="q-pa-xs text-white bg-primary">
    <OctIcon symbol="stack" size="14pt" />
    Stashed Changes
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import OctIcon from "@/components/OctIcon.vue";

export default defineComponent({
  name: "StashedChanges",

  components: {
    OctIcon,
  },
});
</script>
