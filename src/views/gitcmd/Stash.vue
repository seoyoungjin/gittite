<script lang="ts">
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  data() {
    return {
      jsonData: null,
      jsonData2: null,
    };
  },
  components: {
    VueJsonPretty,
  },
  methods: {
    async readSettings() {
      var data = await readTextFile("gittite/settings.json", {
        dir: BaseDirectory.Config,
      });
      return JSON.parse(data);
    },
  },
  async mounted() {
    this.jsonData = await invoke("get_settings");
    this.jsonData2 = await this.readSettings();
  },
};
</script>

<template>
  <q-page class="q-ma-lg">
    <h5>Git Stash</h5>

    Stash the changes in a dirty working directory away stashing your work
    <pre>
git stash
git save [message]
    </pre>

    list
    <pre>
git stash list
    </pre>

    view stash diff
    <pre>
git stash show
git stash show -p
    </pre>

    creating a branch from your stash
    <pre>
git stash branch [branchnamel [stash]
    </pre>

    cleaning up your stash
    <pre>
git stash drop [stash]
git stash clear
    </pre>

    re-applying your stashed changes
    <pre>
git stash [pop | apply] [stash]
    </pre>
  </q-page>
</template>
