<template>
  <q-icon :name="octIconForStatus(status)" :color="colorForStatus(status)" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import {
  octDiff16,
  octDiffAdded16,
  octDiffIgnored16,
  octDiffModified16,
  octDiffRemoved16,
  octDiffRenamed16,
  octFileDiff16,
} from "quasar-extras-svg-icons/oct-icons-v17";

export default defineComponent({
  name: "OctStatusIcon",

  setup() {
    return {
      octDiff16,
      octDiffAdded16,
      octDiffIgnored16,
      octDiffModified16,
      octDiffRemoved16,
      octDiffRenamed16,
      octFileDiff16,
    };
  },

  props: {
    status: {
      type: String,
      default: "New",
    },
  },
  methods: {
    octIconForStatus(status: string | undefined): any {
      // alert(status);
      switch (status) {
        case "New":
        case "Added":
        case "Untracked":
          return octDiffAdded16;
        case "Modified":
          return octDiffModified16;
        case "Deleted":
          return octDiffRemoved16;
        case "Renamed":
          return octDiffRenamed16;
        case "Conflicted":
        default:
          return null;
      }
    },

    colorForStatus(status: string | undefined): string {
      switch (status) {
        case "New":
        case "Added":
        case "Untracked":
          return "green";
        case "Modified":
          return "yellow-9";
        case "Deleted":
          return "red";
        case "Renamed":
          return "blue";
        // case "Conflicted":
        default:
          return "grey";
      }
    },
  },
});
</script>
