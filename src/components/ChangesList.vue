<template>
  <div class="q-ma-none">
    <!--Unstaged-->
    <div class="text-h7">Unstaged Changes</div>
    <q-scroll-area style="height: 25vh">
      <q-list dense bordered padding class="rounded-borders">
        <q-item
          v-for="(item, index) in unstagedData"
          :key="index"
          clickable
          @click="clickItem(item)"
        >
          <q-item-section side>
            <q-icon
              :name="octIconForStatus(item.wtree)"
              color="blue"
              size="14px"
            />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ item.path }}</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-scroll-area>

    <!--Staged-->
    <div class="text-h7">Staged Changes</div>
    <q-scroll-area style="height: 25vh">
      <q-list dense bordered padding class="rounded-borders">

        <q-item
          v-for="(item, index) in stagedData"
          :key="index"
          clickable
          @click="clickItem(item)"
        >
          <q-item-section side>
            <q-icon
              :name="octIconForStatus(item.stage)"
              color="amber"
              size="14px"
            />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ item.path }}</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-scroll-area>

    <div>
      <ChangesOption />
    </div>
  </div>
</template>

<script lang="ts">
import ChangesOption from "@/components/ChangesOption.vue";
import {
  octDiff16,
  octDiffAdded16,
  octDiffIgnored16,
  octDiffModified16,
  octDiffRemoved16,
  octDiffRenamed16,
  octFileDiff16,
} from "quasar-extras-svg-icons/oct-icons-v17";
import * as git2rs from "@/api/git2rs";

export default {
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

  components: {
    ChangesOption,
  },

  mounted() {
    this.refreshStatus();
  },

  data() {
    return {
      stagedData: [],
      unstagedData: [],
    };
  },

  methods: {
    refreshStatus() {
      (async () => {
        this.stagedData = await git2rs.getStatus("stage");
        this.unstagedData = await git2rs.getStatus("workdir");
      })();
    },

    clickItem(item) {
      this.$emit("selectFile", item.path);
    },

    octIconForStatus(status: string): string {
      switch (status) {
        case "New":
        case "Added":
        case "Untracked":
          return this.octDiffAdded16;
        case "Modified":
          return this.octDiffModified16;
        case "Deleted":
          return this.octDiffRemoved16;
        case "Renamed":
          return this.octDiffRenamed16;
        // case "Conflicted":
        default:
          throw "Unknown status";
      }
    },
  },
};
</script>
