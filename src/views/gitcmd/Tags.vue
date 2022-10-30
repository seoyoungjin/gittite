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
    <h5>Git Tag</h5>

    creating a tag
    <pre>
git tag [tagname]
    </pre>

    annotated tag
    <pre>
git tag -a v1.4 -m "my version 1.4"
    </pre>

    list tag
    <pre>
git tag
    </pre>

    tagging old commits
    <pre>
git tag [tagname] [commit hash]
    </pre>

    retagging/replacing old tags
    <pre>
git tag -a -f v1.4 15027957951
    </pre>

    push tags to remote
    <pre>
git push origin v1.4
    </pre>

    checkout tag
    <pre>
git checkout v1.4
    </pre>

    delete tag
    <pre>
git tag -d [tagname]
    </pre>

    filttering tags
    <pre>
git tag -l [pattern]
    </pre>

  </q-page>
</template>
