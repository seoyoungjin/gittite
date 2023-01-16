<template>
  <q-icon :name="octIconForSymbol(symbol)" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import {
  octRepoClone16,
  octLightBulb16,
  octSquare16,
  octFileDirectory16,
  octPlus16,
} from "quasar-extras-svg-icons/oct-icons-v17";

export default defineComponent({
  name: "OctIcon",

  setup() {
    return {
      octRepoClone16,
      octLightBulb16,
      octSquare16,
      octFileDirectory16,
      octPlus16,
    };
  },

  props: {
    symbol: {
      type: String,
      default: "",
    },
  },
  methods: {
    octIconForSymbol(symbol: string | undefined): any {
      // alert(symbol);
      switch (symbol) {
        case "repoClone":
          return octRepoClone16;
        case "lightBulb":
          return octLightBulb16;
        case "plus":
          return octPlus16;
        case "fileDriectory":
          return octFileDirectory16;
        default:
          return octSquare16;
      }
    },
  },
});
</script>
