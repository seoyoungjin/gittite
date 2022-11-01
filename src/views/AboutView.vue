<script>
import { getName, getVersion, getTauriVersion } from "@tauri-apps/api/app";
import { exit } from "@tauri-apps/api/process";
import { ask } from "@tauri-apps/api/dialog";

export default {
  data() {
    return {
      appName: "Unknown",
      appVersion: "0.0.0!",
      tauriVersion: "0.0.0",
    };
  },
  methods: {
    async closeApp() {
      const yes = await ask("Are you sure?", "Tauri");
      if (yes) {
        await exit();
      }
    },
  },
  mounted() {
    getName().then((n) => {
      this.appName = n;
    });
    getVersion().then((v) => {
      this.appVersion = v;
    });
    getTauriVersion().then((v) => {
      this.tauriVersion = v;
    });
    // this.appName = await getName();
    // this.appVersion = await getVersion();
    // this.tauriVersion = await getTauriVersion();
  },
};
</script>

<template>
  <q-page class="q-ma-lg">
    <h5>Welcome</h5>
    <p>
      This is a demo of Tauri's API capabilities using the
      <code>@tauri-apps/api</code> package.
    </p>

    <pre>
    App name : <code>{{ appName }}</code>
    App version : <code>{{ appVersion }}</code>
    tauriVersion : <code>{{ tauriVersion }}</code>
    </pre>

    <div class="flex flex-wrap gap-1 shadow-">
      <button v-on:click="closeApp">Close application</button>
    </div>
  </q-page>
</template>
