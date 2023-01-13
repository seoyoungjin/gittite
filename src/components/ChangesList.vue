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
              <OctStatusIcon
                v-bind:status="item.wtree"
                size="14pt"
                @click="stageFile(item)"
              />
            </q-item-section>
            <q-item-section>
              <q-item-label><PathLabel v-bind:path="item.path" /></q-item-label>
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
              <OctStatusIcon
                v-bind:status="item.status"
                size="14pt"
                @click="unstageFile(item)"
              />
            </q-item-section>
            <q-item-section>
              <q-item-label><PathLabel v-bind:path="item.path" /></q-item-label>
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
import { useCommitStageStore } from "@/stores/commit-stage";
// import ChangesOption from "@/components/ChangesOption.vue";
import CommitMessage from "@/components/CommitMessage.vue";
import OctStatusIcon from "@/components/OctStatusIcon.vue";
import PathLabel from "@/components/PathLabel.vue";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  data() {
    return {
      stageStyle: { height: "calc(100%-230pt)" },
    };
  },

  components: {
    // ChangesOption,
    CommitMessage,
    OctStatusIcon,
    PathLabel,
  },

  computed: {
    ...mapState(useCommitStageStore, ["allStagedFiles", "allUnstagedFiles"]),
  },

  methods: {
    ...mapActions(useCommitStageStore, ["updateCommitStage", "setCurrentItem"]),

    onChildResize(size: any) {
      // alert(JSON.stringify(size));
      // (this.$q.screen.height - 90 - size.height) + "px";
      this.stageStyle.height = "calc(100% - " + (size.height + 5) + "pt)";
    },

    clickItem(item: any) {
      this.setCurrentItem(item);
    },

    stageFile(item: any) {
      let stage_function: (path: string) => Promise<boolean>;

      if (item.wtree == "Deleted") {
        stage_function = git2rs.stageRemovePath;
      } else {
        stage_function = git2rs.stageAddPath;
      }
      stage_function(item.path)
        .then(async () => {
          await this.updateCommitStage();
        })
        .catch(async (e) => {
          var message = JSON.stringify(e, null, 4);
          await this.showNotification(message);
        });
    },

    unstageFile(item: any) {
      git2rs
        .resetStage(item.path)
        .then(async () => {
          await this.updateCommitStage();
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
  },
});
</script>
