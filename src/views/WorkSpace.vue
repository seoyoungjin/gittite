<template>
  <q-layout view="hHh lpR fFf" class="bg-grey-1">
    <q-splitter
      v-model="splitterModel"
      :limits="[280, 600]"
      unit="px"
      before-class="overflow-hidden"
      after-class="overflow-hidden"
      style="height: 100vh"
    >
      <template v-slot:before>
        <ToolBar />
        <q-tabs
          v-model="tab"
          dense
          no-caps
          class="bg-orange text-black shadow-2"
        >
          <q-tab name="changes" label="Changes" />
          <q-tab name="history" label="History" />
        </q-tabs>

        <q-separator />

        <q-tab-panels v-model="tab" animated class="fit">
          <q-tab-panel name="changes" class="q-pa-none">
            <ChangesList @discardChanges="onDiscardChanges" />
          </q-tab-panel>

          <q-tab-panel name="history" class="q-pa-xs">
            <HistoryList />
          </q-tab-panel>
        </q-tab-panels>
      </template>

      <template v-slot:after>
        <ToolBar2
          @initRepository="showInitReposity = true"
          @addLocalRepository="showAddLocalReposity = true"
          @cloneRepository="showCloneReposity = true"
          @preference="showPreference = true"
          @branchSwitch="onBranchSwitch"
        />
        <div v-if="tab === 'changes'" class="q-pa-none">
          <ContentForChanges />
        </div>
        <div v-else class="q-pa-none fit">
          <ContentForHistory />
        </div>
      </template>
    </q-splitter>
  </q-layout>

  <InitRepository v-model="showInitReposity" />
  <AddLocalRepository v-model="showAddLocalReposity" />
  <CloneRepository v-model="showCloneReposity" />
  <Preference v-model="showPreference" />
  <RepoSettings v-model="showRepoSettings" />
  <BranchCreate v-model="showBranchCreate" @branchSwitch="onBranchSwitch" />
  <BranchRename v-model="showBranchRename" />
  <BranchDelete v-model="showBranchDelete" />
  <BranchReset v-model="showBranchReset" />
  <BranchSwitch v-model="showBranchSwitch" @ok="branchSwitch" />
</template>

<script lang="ts">
import { ref } from "vue";
import { defineComponent } from "vue";
import { listen } from "@tauri-apps/api/event";
import ToolBar from "@/components/toolbar/ToolBar.vue";
import ToolBar2 from "@/components/toolbar/ToolBar2.vue";
import ChangesList from "@/components/changes/ChangesList.vue";
import ContentForChanges from "@/components/changes/ContentForChanges.vue";
import HistoryList from "@/components/history/HistoryList.vue";
import ContentForHistory from "@/components/history/ContentForHistory.vue";
// dialog
import InitRepository from "@/components/dialog/InitRepository.vue";
import AddLocalRepository from "@/components/dialog/AddLocalRepository.vue";
import CloneRepository from "@/components/dialog/CloneRepository.vue";
import Preference from "@/components/dialog/Preference.vue";
import RepoSettings from "@/components/dialog/RepoSettings.vue";
import BranchCreate from "@/components/dialog/BranchCreate.vue";
import BranchRename from "@/components/dialog/BranchRename.vue";
import BranchDelete from "@/components/dialog/BranchDelete.vue";
import BranchReset from "@/components/dialog/BranchReset.vue";
import BranchSwitch from "@/components/dialog/BranchSwitch.vue";
import { useRepositoryStore } from "@/stores/repository";
import { useSettingsStore } from "@/stores/settings";
import { useCommitStageStore } from "@/stores/commit-stage";
import { useAppStore } from "@/stores/app";
import type { StatusItem } from "@/models/status";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  setup() {
    const repoStore = useRepositoryStore();
    const settingsStore = useSettingsStore();
    const stageStore = useCommitStageStore();
    const appStore = useAppStore();

    settingsStore.loadSettings();
    appStore.initStore();

    return {
      appStore,
      repoStore,
      stageStore,
      splitterModel: ref(280),
      tab: ref("changes"),
    };
  },

  async mounted() {
    listen("menu-event", (ev) => {
      if (ev.payload == "init") {
        this.showInitReposity = true;
      } else if (ev.payload == "add-local") {
        this.showAddLocalReposity = true;
      } else if (ev.payload == "clone") {
        this.showCloneReposity = true;
      } else if (ev.payload == "preference") {
        this.showPreference = true;
      } else if (ev.payload == "repo-settings") {
        this.showRepoSettings = true;
      } else if (ev.payload == "branch-create") {
        this.showBranchCreate = true;
      } else if (ev.payload == "branch-rename") {
        this.showBranchRename = true;
      } else if (ev.payload == "branch-delete") {
        this.showBranchDelete = true;
      } else if (ev.payload == "branch-reset") {
        this.showBranchReset = true;
      } else if (ev.payload == "branch-stash") {
        this.onStashAllChanges();
      } else if (ev.payload == "select") {
        if (this.$route.path != "/select") {
          this.$router.push("/select");
        }
      }
      console.log(ev.payload);
    });

    await this.repoStore.loadRepositoryInfo();
    if (!this.repoStore.currentRepository) {
      this.$router.push("/select");
    }
  },

  data() {
    return {
      // dialog
      showInitReposity: false,
      showAddLocalReposity: false,
      showCloneReposity: false,
      showPreference: false,
      showRepoSettings: false,
      showBranchCreate: false,
      showBranchRename: false,
      showBranchDelete: false,
      showBranchReset: false,
      showBranchSwitch: false,
    };
  },

  components: {
    ToolBar,
    ToolBar2,
    ChangesList,
    HistoryList,
    ContentForChanges,
    ContentForHistory,
    // dialog
    InitRepository,
    AddLocalRepository,
    CloneRepository,
    Preference,
    RepoSettings,
    BranchCreate,
    BranchRename,
    BranchDelete,
    BranchReset,
    BranchSwitch,
  },

  methods: {
    setRepository(path: string) {
      this.repoStore.setRepository(path);
    },

    async branchSwitch() {
      const branchName = this.repoStore.branchToSwitch;
      const info = this.repoStore.getBranchInfo(branchName);
      git2rs
        .checkoutBranch(info.reference)
        .then(() => {
          var message = "Switch to " + branchName;
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
          // refresh
          this.repoStore.loadRepositoryInfo();
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });
    },

    async onBranchSwitch(branchName: string) {
      console.log("onBranchSwithch", branchName);
      await this.repoStore.setBranchToSwitch(branchName);
      if (this.stageStore.hasChanges) {
        this.showBranchSwitch = true;
      } else {
        await this.branchSwitch();
      }
    },

    async onDiscardChanges(item: StatusItem) {
      console.log("onDiscardChanges", item.path);
      this.appStore.setDiscardItem(item);
      this.showBranchReset = true;
    },

    async onStashAllChanges() {
      console.log("onStashAllChanges");
      git2rs
        .stashSave("Gittite", true, false)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
          this.repoStore.loadRepositoryInfo();
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });
    },
  },
});
</script>
