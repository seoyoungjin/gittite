<script lang="ts">
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";
import * as git2rs from "../../api/git2rs";

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
    this.jsonData = await git2rs.loadSettings();
    this.jsonData2 = await this.readSettings().catch((e) => {
      return e;
    });
  },
};
</script>

<template>
  <q-page class="q-ma-lg">
    <h5>Settings</h5>

    We can read settings file with two methods.
    <br />
    <br />
    1. rust with tauri command
    <br />
    2. javascript using "@tauri-apps/api/fs"

    <h6>1. via tauri::command</h6>
    <div v-if="jsonData">
      <vue-json-pretty :data="jsonData" />
    </div>
    <br />

    <h6>2. with Javascript</h6>
    <div v-if="jsonData2">
      <vue-json-pretty :data="jsonData2" />
    </div>
  </q-page>
</template>
