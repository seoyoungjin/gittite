<template>
  <q-btn flat no-caps loader @click="simulateProgress">
    <q-inner-loading :showing="visible">
      <q-spinner class="on-left" />
    </q-inner-loading>
    <q-item-section align="left">
      <q-item-label>
        Fetch
      </q-item-label>
      <q-item-label>
        <small>not yet</small>
      </q-item-label>
    </q-item-section>
  </q-btn>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { useLayoutStore } from "@/stores/layout.js";

export default defineComponent({
  setup () {
    const visible = ref(false)

    return {
      visible,

      simulateProgress (event, done) {
        visible.value = true;
        setTimeout(() => {
          visible.value = false;
          done()
        }, 3000)
      }
    }
  },

  methods: {
  },
});
</script>
