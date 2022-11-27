<template>
  <q-header elevated class="bg-white text-grey-8 q-py-xs" style="height: 50px">
    <q-toolbar>
      <q-btn flat dense icon="menu" @click="toggleLeftDrawer" />
      <q-btn flat no-caps no-wrap class="q-ml-xs" v-if="$q.screen.gt.xs">
        <img alt="Vue logo" src="@/assets/logo.svg" width="25" height="25" />
        <q-toolbar-title shrink class="text-weight-bold">
          Gittite
        </q-toolbar-title>
      </q-btn>

      <q-separator vertical />
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
  </q-header>

  <InitRepository v-model="showInitReposityDialog"/>
</template>

<script lang="ts">
import SetLayout from "../components/SetLayout.vue";
import InitRepository from "../components/dialog/InitRepository.vue";

export default {
  name: "ToolBar",

  components: {
    SetLayout,
    InitRepository,
  },

  data() {
    const leftDrawerOpen = true;
    // dialogs
    const showInitReposityDialog = false;
    return {
      leftDrawerOpen,
      showInitReposityDialog,
    };
  },

  methods: {
    toggleLeftDrawer() {
      this.leftDrawerOpen = !this.leftDrawerOpen;
      this.$emit("showLeftDrawer", this.leftDrawerOpen);
    },

    onInitRepository() {
      this.showInitReposityDialog = true;
    }
  },
};
</script>
