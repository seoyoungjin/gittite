<template>
  <q-page class="q-ma-lg">
    <div class="about">
      <h6>Log</h6>
    </div>

    <q-btn color="primary" no-caps @click="getCommits">Git Logs</q-btn>
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

  methods: {
    getCommits() {
      git2rs
        .getCommits()
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
