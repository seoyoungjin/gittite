<script lang="ts">
import 'vue-json-pretty/lib/styles.css';
import VueJsonPretty from 'vue-json-pretty';
import { invoke } from '@tauri-apps/api/tauri';
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';

export default {
  data() {
    return {
      jsonData: null,
      jsonData2: null,
    }
  },
  components: {
    VueJsonPretty,
  },
  methods: {
    async readSettings() {
      var data = await readTextFile(
        'gittite/settings.json',
        { dir: BaseDirectory.Config}
      );
      return JSON.parse(data);
    }
  },
  async mounted() {
    this.jsonData = await invoke('get_settings');
    this.jsonData2 = await this.readSettings();
  }
}
</script>

<template>
  <q-page class="q-ma-lg">
    <h5> Settings </h5>

    We can read settings file with two methods.
    <br />
    <br />
    1. rust with tauri command
    <br />
    2. javascript using "@tauri-apps/api/fs"

    <h6> 1. via tauri::command </h6>
    <vue-json-pretty :data=jsonData />
    <br />

    <h6> 2. with Javascript </h6>
    <vue-json-pretty :data=jsonData2 />
  </q-page>
</template>
