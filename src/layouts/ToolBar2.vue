<template>
  <q-toolbar class="bg-grey-10 text-grey-2">
    <q-btn flat dense icon="list" @click="onSetRepository">
      <q-tooltip>
        New Repository
      </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="subscriptions" @click="onInitRepository">
      <q-tooltip>
        Add Local Repository
      </q-tooltip>
    </q-btn>
    <q-btn flat dense icon="file_copy" @click="onClone">
      <q-tooltip>
        Clone Repository
      </q-tooltip>
    </q-btn>
    <q-separator vertical />
    <q-btn flat dense icon="settings" @click="onSettings">
      <q-tooltip>
        Settings
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
  },
};
</script>
