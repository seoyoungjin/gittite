<template>
  <q-page class="q-ma-lg">
    <div class="about">
      <h6>Blame</h6>
    </div>

    <q-form id="git-blame">
      <q-input
        v-model="blameForm.filePath"
        label="File"
        hint="Enter filename"
      />
      <q-input
        v-model="blameForm.commitId"
        label="Commit ID"
        hint="Enter Commit ID"
      />
    </q-form>

    <div>
      <q-btn color="primary" no-caps @click="selectFile">Select File</q-btn>
      <q-btn color="primary" no-caps @click="blameFile">Blame</q-btn>
    </div>
    <br />

    <div class="text-h7">Commit files</div>
    <div v-if="commitFiles">
      <vue-json-pretty :data="commitFiles" />
    </div>
    <br />

    <pre>
Todo
- commit_id
- complicated blame format like diff
    </pre>

    <div class="text-h7">Blame File</div>
    <div v-if="resBlame">
      <vue-json-pretty :data="resBlame" />
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { open } from "@tauri-apps/api/dialog";
import * as git2rs from "../../lib/git2rs";

export default defineComponent({
  components: {
    VueJsonPretty,
  },

  mounted() {
    this.refresh();
  },

  data() {
    return {
      blameForm: {
        filePath: "",
        commitId: "",
      },
      commitFiles: null,
      resBlame: null,
    };
  },

  methods: {
    refresh() {
      (async () => {
        // Todo - HEAD
        this.commitFiles = (await git2rs.commitFiles("HEAD")) as any;
      })();
    },

    async selectFile() {
      const selected = await open({
        directory: false,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      this.blameForm.filePath = selected;
    },

    async blameFile() {
      alert(this.blameForm.filePath);
      if (!this.blameForm.filePath) return;
      var path = this.blameForm.filePath;
      // TODO - commitId
      git2rs
        .blameFile(path, null)
        .then((message) => {
          this.resBlame = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resBlame = { error: JSON.stringify(e) } as any;
          }
        });
    },
  },
});
</script>
