<template>
  <v-container fluid fill-height>
    <v-layout align-center justify-center>
      <q-btn round dense flat icon="apps" @click="setLayout('app-layout')">
        <q-tooltip>App</q-tooltip>
      </q-btn>
      <q-btn round dense flat icon="message" @click="setLayout('dev-layout')">
        <q-tooltip>Dev</q-tooltip>
      </q-btn>
    </v-layout>
  </v-container>
</template>

<script>
import { useLayoutStore } from "@/stores/layout.js";

export default {
  setup() {
    const store = useLayoutStore();
    return { store };
  },
  methods: {
    setLayout(layout) {
      if (layout === "app-layout") {
        this.$router.push("/");
      } else if (this.$route.path === "/" || this.$route.path === "/gitcmd") {
        this.$router.push("/gitcmd/home");
      }
      this.store.setLayout(layout);
    },
  },
};
</script>
