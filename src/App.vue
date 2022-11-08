<template>
  <v-app>
    <component v-bind:is="layout"></component>
  </v-app>
</template>

<script>
import { computed } from 'vue';
import { useLayoutStore } from './stores/layout.js';
import AppLayout from './layouts/AppLayout.vue'
import DevLayout from './layouts/DevLayout.vue'

export default {
  setup() {
    const store = useLayoutStore();
    const layout = computed(() => store.layout);
    return {
      layout,
    };
  },
  components: {
    'app-layout': AppLayout,
    'dev-layout': DevLayout
  }
}
</script>
