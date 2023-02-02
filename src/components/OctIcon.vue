<template>
  <q-icon :name="octIconForSymbol(symbol)" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import * as OctIcon from "quasar-extras-svg-icons/oct-icons-v17";

export default defineComponent({
  name: "OctIcon",

  setup() {
    return {
      OctIcon,
    };
  },

  props: {
    symbol: {
      type: String,
      required: true,
    },
  },
  methods: {
    octIconForSymbol(symbol: string | undefined): any {
      switch (symbol) {
        case "alert":
          return OctIcon.octAlert16;
        case "repoClone":
          return OctIcon.octRepoClone16;
        case "lightBulb":
          return OctIcon.octLightBulb16;
        case "plus":
          return OctIcon.octPlus16;
        case "fileDriectory":
          return OctIcon.octFileDirectory16;
        case "gitCommit":
          return OctIcon.octGitCommit16;
        case "diff":
          return OctIcon.octDiff16;
        case "tag":
          return OctIcon.octTag16;
        case "stack":
          return OctIcon.octStack16;
        default:
          return OctIcon.octDot16;
      }
    },
  },
});
</script>
