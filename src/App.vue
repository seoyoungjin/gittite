<template>
  <v-app>
    <component v-bind:is="layout"></component>
  </v-app>
</template>

<script>
import AppLayout from './layouts/AppLayout.vue'
import DevLayout from './layouts/DevLayout.vue'

export default {
  computed: {
    layout () {
      // return this.$store.getters.layout
      return 'dev-layout';
    }
  },
  components: {
    'app-layout': AppLayout,
    'dev-layout': DevLayout
  }
}
</script>