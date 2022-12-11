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
  },
};
</script>
