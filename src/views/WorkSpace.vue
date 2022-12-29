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
          <q-tab-panel name="changes" class="q-pa-sm">
            <ChangesList v-on:selectItem="handleSelectItem" />
          </q-tab-panel>

          <q-tab-panel name="history" class="q-pa-sm">
            <history-list v-on:selectItem="handleSelectItem" />
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
        <q-scroll-area class="fit">
          <div class="q-pa-md">
            <diff-view :curSelected="curSelected" />
          </div>
        </q-scroll-area>
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
import HistoryList from "@/components/HistoryList.vue";
import DiffView from "@/components/DiffView.vue";
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
import { usePropStore } from "@/stores/props";

export default defineComponent({
  setup() {
    const repoStore = useRepositoryStore();
    const settingsStore = useSettingsStore();
    const propStore = usePropStore();

    settingsStore.loadSettings();
    propStore.initStore();

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
      } else if (ev.payload == "add_local") {
        this.showAddLocalReposity = true;
      } else if (ev.payload == "clone") {
        this.showCloneReposity = true;
      } else if (ev.payload == "preference") {
        this.showPreference = true;
      } else if (ev.payload == "branch_create") {
        this.showBranchCreate = true;
      } else if (ev.payload == "branch_rename") {
        this.showBranchRename = true;
      } else if (ev.payload == "branch_delete") {
        this.showBranchDelete = true;
      } else if (ev.payload == "branch_reset") {
        this.showBranchReset = true;
      }
      console.log(ev.payload);
    });

    await this.repoStore.loadRepositoryInfo();
    // alert(this.repoStore.repositoryName);
    if (!this.repoStore.repositoryName) {
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
    DiffView,
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
