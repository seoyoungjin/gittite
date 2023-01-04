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
    <q-space />

    <div class="q-gutter-sm row items-center no-wrap">
      <SetLayout />
    </div>
  </q-toolbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import { octGitBranch16 } from "quasar-extras-svg-icons/oct-icons-v17";
import SetLayout from "../components/SetLayout.vue";
import * as git2rs from "@/api/git2rs";

export default defineComponent({
  name: "ToolBar2",

  setup() {
    return {
      octGitBranch16,
    };
  },

  components: {
    SetLayout,
  },

  computed: {
    ...mapState(useRepositoryStore, ["currentBranch", "allBranches"]),
  },

  methods: {
    ...mapActions(useRepositoryStore, ["getBranchInfo", "loadRepositoryInfo"]),

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
    onRefresh() {
      this.loadRepositoryInfo();
    },
    onBranchSwitch(branchName: string) {
      // TODO check changes
      var info = this.getBranchInfo(branchName);
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
          this.loadRepositoryInfo();
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
