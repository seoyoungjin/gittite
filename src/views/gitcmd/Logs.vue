<template>
  <q-page class="q-ma-lg">
    <div class="about">
      <h6>Log</h6>
    </div>

    <q-btn color="primary" no-caps @click="getCommits"> Git Logs</q-btn>
    <br />

    <div>
      <vue-json-pretty :data=response />
    </div>
  </q-page>
</template>

<script lang="ts">
import 'vue-json-pretty/lib/styles.css';
import VueJsonPretty from 'vue-json-pretty';
import { invoke } from '@tauri-apps/api/tauri';

export default {
  components: {
    VueJsonPretty,
  },
  data() {
    return {
      response: null
    }
  },

  methods: {
    getCommits() {
      invoke('get_commits').then((message) => {
        // alert("Got response. Iterationg....");
        this.response = message;
      }).catch((e) => {
        if (typeof e == 'string') {
          this.response = {"error": e};
        } else {
          this.response = {"error": JSON.stringify(e)};
        }
      });
    }
  }
}
</script>
