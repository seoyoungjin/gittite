<template>
  <q-page class="q-ma-lg">
    <div class="about">
      <h6>Select Repository</h6>
    </div>

    <q-btn color="primary" no-caps @click="selectRepo">Confirm</q-btn>
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
import { open } from "@tauri-apps/api/dialog";
import * as git2rs from "../../api/git2rs";

export default defineComponent({
  components: {
    VueJsonPretty,
  },

  data() {
    return {
      response: null,
    };
  },

  mounted() {},

  methods: {
    async selectRepo() {
      const selected = await open({
        directory: true,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      git2rs
        .setRepository(selected)
        .then((repo) => {
          this.response = {
            "Selected Directory": selected,
            "Current repository": repo,
          } as any;
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
