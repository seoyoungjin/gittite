<template>
  <q-toolbar class="bg-grey-10 text-grey-2 q-pa-none">
    <q-btn-dropdown flat no-caps>
      <template v-slot:label>
        <div class="row items-center no-wrap">
          <q-icon :name="octGitBranch16" size="16pt" class="q-pa-sm" />
          <q-item-section align="left">
            <q-item-label>
              <small>Current Branch</small>
            </q-item-label>
            <q-item-label>{{ currentBranch }}</q-item-label>
          </q-item-section>
        </div>
      </template>
      <q-list dense>
        <q-item
          v-for="branch in allBranches"
          :key="branch.name"
          clickable
          v-close-popup
          @click="onBranchSwitch(branch.name)"
        >
          <q-icon :name="octGitBranch16" size="14pt" class="q-pa-xs" />
          <q-item-section>
            <q-item-label>{{ branch.name }}</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-btn-dropdown>

    <PushPullBtn />

    <div class="row no-wrap" v-if="$q.screen.gt.sm">
    <q-btn flat dense icon="subscriptions" @click="onInitRepository">
      <q-tooltip> New Repository </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="list" @click="onAddLocalRepository">
      <q-tooltip> Add Local Repository </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="file_copy" @click="onCloneRepository">
      <q-tooltip> Clone Repository </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="refresh" @click="onRefresh">
      <q-tooltip> Refresh </q-tooltip>
    </q-btn>
    <q-separator vertical />
    <q-btn flat dense icon="settings" @click="onPreference">
      <q-tooltip> Preference </q-tooltip>
    </q-btn>
    </div>

    <q-space />

    <div class="row no-wrap" v-if="$q.screen.gt.sm">
      <SetLayout />
    </div>
  </q-toolbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import { octGitBranch16 } from "quasar-extras-svg-icons/oct-icons-v17";
import PushPullBtn from "./PushPullBtn.vue";
import SetLayout from "./SetLayout.vue";

export default defineComponent({
  name: "ToolBar2",

  setup() {
    return {
      octGitBranch16,
    };
  },

  components: {
    PushPullBtn,
    SetLayout,
  },

  computed: {
    ...mapState(useRepositoryStore, ["currentBranch", "allBranches"]),
  },

  methods: {
    ...mapActions(useRepositoryStore, ["loadRepositoryInfo"]),

    onInitRepository() {
      this.$emit("initRepository");
    },
    onAddLocalRepository() {
      this.$emit("addLocalRepository");
    },
    onCloneRepository() {
      this.$emit("cloneRepository");
    },
    onPreference() {
      this.$emit("preference");
    },
    onBranchSwitch(branchName: string) {
      if (this.currentBranch !== branchName)
        this.$emit("branchSwitch", branchName);
    },
    onRefresh() {
      this.loadRepositoryInfo();
    },
  },
});
</script>
