<template>
  <v-container fluid fill-height>
    <v-layout align-center justify-center>
      <q-btn
        round
        dense
        flat
        color="grey-8"
        icon="apps"
        @click="store.setLayout('app-layout')"
      >
        <q-tooltip>App</q-tooltip>
      </q-btn>
      <q-btn
        round
        dense
        flat
        color="grey-8"
        icon="message"
        @click="store.setLayout('dev-layout')"
      >
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
};
</script>
