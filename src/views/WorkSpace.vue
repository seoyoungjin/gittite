<template>
  <q-layout view="hHh lpR fFf" class="bg-grey-1">
    <q-splitter
      v-model="splitterModel"
      :limits="[300, 600]"
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
            <ChangesList />
          </q-tab-panel>

          <q-tab-panel name="history" class="q-pa-xs">
            <HistoryList v-on:selectItem="handleSelectItem" />
          </q-tab-panel>
        </q-tab-panels>
      </template>

      <template v-slot:after>
        <ToolBar2
          @initRepository="showInitReposity = true"
          @addLocalRepository="showAddLocalReposity = true"
          @cloneRepository="showCloneReposity = true"
          @preference="showPreference = true"
        />
        <div v-if="tab === 'changes'" class="q-pa-none">
          <ContentForChanges />
        </div>
        <div v-else class="q-pa-none fit">
          <ContentForHistory :selection="curSelected" />
        </div>
      </template>
    </q-splitter>
  </q-layout>

  <InitRepository v-model="showInitReposity" />
  <AddLocalRepository v-model="showAddLocalReposity" />
  <CloneRepository v-model="showCloneReposity" />
  <Preference v-model="showPreference" />
  <BranchCreate v-model="showBranchCreate" />
  <BranchRename v-model="showBranchRename" />
  <BranchDelete v-model="showBranchDelete" />
  <BranchReset v-model="showBranchReset" />
</template>

<script lang="ts">
import { ref } from "vue";
import { defineComponent } from "vue";
import { listen } from "@tauri-apps/api/event";
import ToolBar from "@/layouts/ToolBar.vue";
import ToolBar2 from "@/layouts/ToolBar2.vue";
import ChangesList from "@/components/ChangesList.vue";
// import ChangesOption from "@/components/ChangesOption.vue";
import HistoryList from "@/components/history/HistoryList.vue";
import ContentForChanges from "@/components/ContentForChanges.vue";
import ContentForHistory from "@/components/ContentForHistory.vue";
// dialog
import InitRepository from "@/components/dialog/InitRepository.vue";
import AddLocalRepository from "@/components/dialog/AddLocalRepository.vue";
import CloneRepository from "@/components/dialog/CloneRepository.vue";
import Preference from "@/components/dialog/Preference.vue";
import BranchCreate from "@/components/dialog/BranchCreate.vue";
import BranchRename from "@/components/dialog/BranchRename.vue";
import BranchDelete from "@/components/dialog/BranchDelete.vue";
import BranchReset from "@/components/dialog/BranchReset.vue";
import { useRepositoryStore } from "@/stores/repository";
import { useSettingsStore } from "@/stores/settings";
import { useAppStore } from "@/stores/app";

export default defineComponent({
  setup() {
    const repoStore = useRepositoryStore();
    const settingsStore = useSettingsStore();
    const appStore = useAppStore();

    settingsStore.loadSettings();
    appStore.initStore();

    return {
      repoStore,
      splitterModel: ref(250),
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
      } else if (ev.payload == "branch-create") {
        this.showBranchCreate = true;
      } else if (ev.payload == "branch-rename") {
        this.showBranchRename = true;
      } else if (ev.payload == "branch-delete") {
        this.showBranchDelete = true;
      } else if (ev.payload == "branch-reset") {
        this.showBranchReset = true;
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
      curSelected: Object,
      // dialog
      showInitReposity: false,
      showAddLocalReposity: false,
      showCloneReposity: false,
      showPreference: false,
      showBranchCreate: false,
      showBranchRename: false,
      showBranchDelete: false,
      showBranchReset: false,
    };
  },

  components: {
    ToolBar,
    ToolBar2,
    ChangesList,
    // ChangesOption,
    HistoryList,
    ContentForChanges,
    ContentForHistory,
    // dialog
    InitRepository,
    AddLocalRepository,
    CloneRepository,
    Preference,
    BranchCreate,
    BranchRename,
    BranchDelete,
    BranchReset,
  },

  methods: {
    handleSelectItem(item: any) {
      this.curSelected = item;
    },
    setRepository(path: string) {
      // subdirectory
      // this.repoStore.setRepository("/Users/yjseo/work/tite/src");
      // not existent
      // this.repoStore.setRepository("/foo");
      // not repository
      // this.repoStore.setRepository("/tmp");
      this.repoStore.setRepository(path);
    },
  },
});
</script>
