<template>
  <div class="q-pa-none" style="height: 100%">
    <q-virtual-scroll
      style="height: calc(100% - 25px)"
      :items="allUnstagedFiles"
      bordered
      separator
      v-slot="{ item, index }"
    >
      <q-item
        class="q-pa-none"
        :key="index"
        dense
        clickable
        @click="clickItem(item)"
      >
        <q-item-section side class="q-pa-xs">
          <q-icon
            :name="octIconForStatus(item.wtree)"
            :color="colorForStatus(item.wtree)"
            size="14pt"
            @click="stageFile(item)"
          />
        </q-item-section>
        <q-item-section>
          <q-item-label>{{ item.path }}</q-item-label>
        </q-item-section>
      </q-item>
    </q-virtual-scroll>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import {
  octDiff16,
  octDiffAdded16,
  octDiffIgnored16,
  octDiffModified16,
  octDiffRemoved16,
  octDiffRenamed16,
  octFileDiff16,
} from "quasar-extras-svg-icons/oct-icons-v17";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
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

  data() {
    return {
      stageStyle: { height: "calc(100%-230pt)" },
    };
  },

  components: {
  },

  computed: {
  },

  methods: {
    clickItem(item: any) {
      this.$emit("selectItem", item);
    },

    octIconForStatus(status: string | undefined): any {
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
        // case "Conflicted":
        default:
          throw "Unknown status";
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
