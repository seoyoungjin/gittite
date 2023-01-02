<template>
  <div class="q-pa-none" style="height: 100%">
    <div :style="stageStyle">
      <div style="height: 50%">
        <div class="q-pa-xs bg-grey-2" style="height: 24px">
          Unstaged Changes
        </div>
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
      <div style="height: 50%">
        <div class="q-pa-xs bg-grey-2" style="height: 24px">
          Staged Changes(Will Commit)
        </div>
        <q-virtual-scroll
          style="height: calc(100% - 25px)"
          :items="allStagedFiles"
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
                :name="octIconForStatus(item.stage)"
                :color="colorForStatus(item.stage)"
                size="14pt"
                @click="unstageFile(item)"
              />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ item.path }}</q-item-label>
            </q-item-section>
          </q-item>
        </q-virtual-scroll>
      </div>
    </div>
    <div>
      <CommitMessage @resize="onChildResize" @commit="updateCommitStage" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import { useCommitStageStore } from "@/stores/commitStage";
// import ChangesOption from "@/components/ChangesOption.vue";
import CommitMessage from "@/components/CommitMessage.vue";
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
    // ChangesOption,
    CommitMessage,
  },

  computed: {
    ...mapState(useCommitStageStore, ["allStagedFiles", "allUnstagedFiles"]),
  },

  methods: {
    ...mapActions(useCommitStageStore, ["updateCommitStage"]),

    onChildResize(size: any) {
      // alert(JSON.stringify(size));
      // this.stageStyle.height = (this.$q.screen.height - 90 - size.height) + "px";
      this.stageStyle.height = "calc(100% - " + (size.height + 5) + "pt)";
    },

    clickItem(item: any) {
      this.$emit("selectItem", item);
    },

    stageFile(item: any) {
      git2rs
        .add(item.path)
        .then(() => {
          this.updateCommitStage();
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.showNotification(message);
        });
    },

    unstageFile(item: any) {
      git2rs
        .resetStage(item.path)
        .then(() => {
          this.updateCommitStage();
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.showNotification(message);
        });
    },

    showNotification(message: string) {
      this.$q.notify({
        color: "red-5",
        textColor: "white",
        icon: "warning",
        message: message,
      });
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
