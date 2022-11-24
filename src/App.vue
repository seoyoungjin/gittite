<template>
  <q-layout view="hHh lpR fFf" class="bg-grey-1">
    <tool-bar />
    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import ToolBar from "@/layouts/ToolBar.vue";

export default {
  name: "App",
  components: {
    ToolBar,
  },
};
</script>
