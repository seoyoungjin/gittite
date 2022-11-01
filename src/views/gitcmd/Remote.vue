<template>
  <q-page class="q-ma-lg">
    <h6>Remote</h6>

    <pre>
$ git remote -v
origin   https://github.com/... (fetch)
origin   https://github.com/... (push)
    </pre>

    <q-btn color="primary" no-caps @click="getRemotes()"> Both</q-btn>
    <br />
    <br />

    <div>
      <vue-json-pretty :data="response" />
    </div>
  </q-page>
</template>

<script lang="ts">
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  components: {
    VueJsonPretty,
  },

  data() {
    return {
      response: null,
    };
  },

  methods: {
    getRemotes() {
      invoke("get_remotes")
        .then((message) => {
          this.response = message;
        })
        .catch((e) => {
          if (typeof e == "string") {
            this.response = { error: e };
          } else {
            this.response = { error: JSON.stringify(e) };
          }
        });
    },
  },
};
</script>
