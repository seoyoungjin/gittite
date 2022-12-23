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
              :color="colorForStatus(item.wtree)"
              size="14pt"
              @click="stageFile(item)"
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
              :color="colorForStatus(item.stage)"
              size="14pt"
              @click="unstageFile(item)"
            />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ item.path }}</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-scroll-area>

    <div>
      <commit-message @commit="getStatus" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useQuasar } from "quasar";
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
import type { StatusItem } from "@/api/types";

export default defineComponent({
  setup() {
    const $q = useQuasar();

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
    // ChangesOption,
    CommitMessage,
  },

  data() {
    return {
      branchName: "",
      stagedData: [] as StatusItem[],
      unstagedData: [] as StatusItem[],
    };
  },

  mounted() {
    this.getStatus();
  },

  computed: {
    ...mapState(useCommitStageStore, ["allStagedFiles"]),
  },

  methods: {
    ...mapActions(useCommitStageStore, ["updateStagedFiles"]),

    getStatus() {
      (async () => {
        this.stagedData = (await git2rs.getStatus("stage")) as [];
        this.unstagedData = (await git2rs.getStatus("workdir")) as [];
        // staged list to pinia state
        this.updateStagedFiles(this.stagedData as []);
      })();
    },

    clickItem(item: any) {
      this.$emit("selectItem", item);
    },

    stageFile(item: any) {
      git2rs
        .add(item.path)
        .then(() => {
          this.getStatus();
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
          this.getStatus();
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
