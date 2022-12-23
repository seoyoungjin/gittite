<template>
  <q-page class="q-ma-lg">
    <h6>Status</h6>

    <q-btn color="primary" @click="getStatus('both')"> Both</q-btn>
    <q-btn color="primary" @click="getStatus('stage')"> Stage</q-btn>
    <q-btn color="primary" @click="getStatus('workdir')">Workdir</q-btn>
    <br />
    <br />

    <div v-if="response">
      <vue-json-pretty :data="response" />
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  components: {
    VueJsonPretty,
  },
  data() {
    return {
      response: null,
    };
  },

  methods: {
    getStatus(args: string) {
      invoke("get_status", { statusType: args })
        .then((message) => {
          this.response = message as any;
        })
        .catch((e) => {
          if (e) {
            this.response = { error: JSON.stringify(e) } as any;
          }
        });
    },
  },
});
</script>
