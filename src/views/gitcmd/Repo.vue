<template>
  <q-page class="q-ma-lg">
    <div class="about">
      <h6>Select Repository</h6>
    </div>

    <div class="text-h7">Settings: {{ this.gitdir }}</div>
    <br />

    <q-btn color="primary" no-caps @click="selectRepo">Confirm</q-btn>
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
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import * as git2rs from "../../api/git2rs";

export default {
  components: {
    VueJsonPretty,
  },

  data() {
    return {
      response: null,
      gitdir: null,
    };
  },

  mounted() {
    this.refreshData();
  },

  methods: {
    refreshData() {
      (async () => {
        var settings = await git2rs.loadSettings();
        this.gitdir = settings.repo;
      })();
    },

    async selectRepo() {
      const selected = await open({
        directory: true,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      invoke("set_repo", { args: selected })
        .then((message) => {
          this.response = { "Current repository": selected };
        })
        .catch((e) => {
          if (e) {
            this.response = { error: JSON.stringify(e) };
          }
        });
    },
  },
};
</script>
