<template>
  <q-toolbar class="bg-grey-10 text-grey-2">
    <q-btn flat dense icon="subscriptions" @click="onInitRepository">
      <q-tooltip>
        New Repository
      </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="list" @click="onAddLocalRepository">
      <q-tooltip>
        Add Local Repository
      </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="file_copy" @click="onCloneRepository">
      <q-tooltip>
        Clone Repository
      </q-tooltip>
    </q-btn>
    <q-separator vertical />
    <q-btn flat dense icon="settings" @click="onPreference">
      <q-tooltip>
        Preference
      </q-tooltip>
    </q-btn>
    <q-space />

    <div class="q-gutter-sm row items-center no-wrap">
      <SetLayout />
    </div>
  </q-toolbar>
</template>

<script lang="ts">
import SetLayout from "../components/SetLayout.vue";

export default {
  name: "ToolBar2",

  components: {
    SetLayout,
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
