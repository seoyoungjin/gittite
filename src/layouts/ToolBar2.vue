<template>
  <q-toolbar class="bg-grey-10 text-grey-2">
    <q-btn flat no-caps class="q-pl-none">
      <!--
      <q-icon name="star" />
      -->
      <q-icon :name="octGitBranch16" size="16pt" class="q-pa-sm" />
      <q-item-section align="left">
        <q-item-label>
          <small>Current Branch</small>
        </q-item-label>
        <q-item-label>{{ currentBranch }}</q-item-label>
      </q-item-section>
    </q-btn>

    <q-btn flat dense icon="subscriptions" @click="onInitRepository">
      <q-tooltip> New Repository </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="list" @click="onAddLocalRepository">
      <q-tooltip> Add Local Repository </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="file_copy" @click="onCloneRepository">
      <q-tooltip> Clone Repository </q-tooltip>
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
import { mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import { octGitBranch16 } from "quasar-extras-svg-icons/oct-icons-v17";
import SetLayout from "../components/SetLayout.vue";

export default {
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
    ...mapState(useRepositoryStore, ["currentBranch"]),
  },

  methods: {
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
  },
};
</script>
