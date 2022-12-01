<template>
  <q-header elevated class="bg-white text-grey-8 q-py-xs" style="height: 50px">
    <q-toolbar>
      <q-btn
        flat
        dense
        round
        @click="toggleLeftDrawer"
        aria-label="Menu"
        icon="menu"
      />
      <q-btn flat no-caps no-wrap class="q-ml-xs" v-if="$q.screen.gt.xs">
        <img alt="Vue logo" src="@/assets/logo.svg" width="25" height="25" />
        <q-toolbar-title shrink class="text-weight-bold">
          Gittite
        </q-toolbar-title>
      </q-btn>

      <q-space />

      <div class="q-gutter-sm row items-center no-wrap">
        <SetLayout />
      </div>
    </q-toolbar>
  </q-header>
</template>

<script lang="ts">
import SetLayout from "@/components/SetLayout.vue";

export default {
  name: "ToolBar",

  components: {
    SetLayout,
  },

  data() {
    const leftDrawerOpen = true;
    return {
      leftDrawerOpen,
    };
  },

  methods: {
    toggleLeftDrawer() {
      this.leftDrawerOpen = !this.leftDrawerOpen;
      this.$emit("showLeftDrawer", this.leftDrawerOpen);
    },
  },
};
</script>
