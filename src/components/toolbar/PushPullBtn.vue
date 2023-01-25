<template>
  <q-btn flat no-caps loader @click="simulateProgress">
    <div style="width: 25pt">
      <q-icon name="sync" v-if="!visible" />
      <q-spinner v-if="visible" />
    </div>
    <q-item-section align="left">
      <q-item-label> Fetch </q-item-label>
      <q-item-label>
        <small>TBD</small>
      </q-item-label>
    </q-item-section>
  </q-btn>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  setup() {
    const visible = ref(false);

    async function simulateProgress() {
      visible.value = true;
      // setTimeout(() => {
      //   visible.value = false;
      // }, 3000);
      await git2rs.fetchAll();
      visible.value = false;
    }

    return {
      visible,
      simulateProgress,
    };
  },

  methods: {},
});
</script>
