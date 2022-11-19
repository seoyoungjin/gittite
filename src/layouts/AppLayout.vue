<template>
  <q-layout view="hHh lpR fFf" class="bg-grey-1">
    <q-header
      elevated
      class="bg-white text-grey-8 q-py-xs"
      style="height: 50px"
    >
      <q-toolbar>
        <q-btn flat no-caps no-wrap class="q-ml-xs" v-if="$q.screen.gt.xs">
          <img alt="Vue logo" src="@/assets/logo.svg" width="25" height="25" />
          <!--
          <q-icon :name="menu" color="red" size="28px" />
          --->
          <q-toolbar-title shrink class="text-weight-bold">
            Gittite
          </q-toolbar-title>
        </q-btn>

        <q-space />

        <div class="q-gutter-sm row items-center no-wrap">
          <set-layout />
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import SetLayout from "@/components/SetLayout.vue";

export default {
  name: "AppLayout",
  components: {
    "set-layout": SetLayout,
  },
};
</script>
