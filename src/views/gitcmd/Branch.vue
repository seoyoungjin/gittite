<script lang="ts">
import 'vue-json-pretty/lib/styles.css';
import VueJsonPretty from 'vue-json-pretty';
import { invoke } from '@tauri-apps/api/tauri';

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
    <h5>Git Branch</h5>

    create a branch
    <pre>
git branch [branchname]
    </pre>

    list branches
    <pre>
git branch
git branch -a
    </pre>

    delete branches
    <pre>
git branch -d [branchname]
git branch -D [branchname]
    </pre>

    checkout
    <pre>
git checkout -b [branchname]
    </pre>

  </q-page>
</template>
