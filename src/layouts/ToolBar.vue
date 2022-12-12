<template>
  <q-toolbar class="bg-grey-10 text-grey-2">
    <q-btn flat no-caps no-wrap class="q-pa-xs">
      <!--
      <img alt="Vue logo" src="@/assets/logo.svg" width="25" height="25" />
      -->
      <q-toolbar-title shrink class="text-weight-bold">
        Gittite
      </q-toolbar-title>
    </q-btn>

    <q-space />

    <q-btn flat no-caps class="q-pa-none">
      <!--
      <q-icon name="star" />
      -->
      <q-item-section align=right>
        <q-item-label>
          <small>Current Repository</small>
        </q-item-label>
        <q-item-label>gittite</q-item-label>
      </q-item-section>
    </q-btn>
  </q-toolbar>
</template>

<script lang="ts">
export default {
  name: "ToolBar",

  data() {
    return {};
  },

  methods: {},
};
</script>
