<template>
  <q-toolbar class="bg-grey-10 text-grey-2">
    <q-btn flat dense icon="subscriptions" @click="onSetRepository" />
    <q-btn flat dense icon="list" @click="onInitRepository" />
    <q-btn flat dense icon="file_copy" @click="onClone" />
    <q-separator vertical />
    <q-btn flat dense icon="settings" @click="onSettings" />
    <q-space />

    <div class="q-gutter-sm row items-center no-wrap">
      <SetLayout />
    </div>
  </q-toolbar>

  <InitRepository v-model="showInitReposityDialog" />
</template>

<script lang="ts">
import SetLayout from "../components/SetLayout.vue";
import InitRepository from "../components/dialog/InitRepository.vue";

export default {
  name: "ToolBar2",

  components: {
    SetLayout,
    InitRepository,
  },

  data() {
    // dialogs
    const showInitReposityDialog = false;
    return {
      showInitReposityDialog,
    };
  },

  methods: {
    onInitRepository() {
      this.showInitReposityDialog = true;
    },
  },
};
</script>
