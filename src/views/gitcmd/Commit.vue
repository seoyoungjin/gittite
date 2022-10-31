<template>
  <q-page class="q-ma-lg">
    <div class="about">
      <h6>Commit</h6>
    </div>

    <q-btn color="primary" no-caps @click="getCommitInfo">Commit Info</q-btn>
    <br />
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

    getCommitInfo() {
      invoke('commit_info', {args: "5f72436b8e421b94114fadc0e27d1e30f0e6407e"}).then((message) => {
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
