<template>
  <q-page class="q-ma-lg">
    <h6>Stage Add/Remove</h6>

    <q-card>
      <q-card-section>
        <q-form @reset="onReset" class="q-gutter-md">
          <q-input v-model="file" label="File to add" hint="Enter file name" />
          <q-btn label="Add" @click="onGitAdd" color="primary" />
          <q-btn label="Unstage" @click="onGitReset" color="primary" />
          <q-btn label="Reset" type="reset" color="primary" />
        </q-form>
      </q-card-section>
    </q-card>

    <br />

    <div v-if="response">
      <vue-json-pretty :data="response" />
    </div>
    <br />

    <div>
      <q-splitter v-model="splitterModel">
        <template v-slot:before>
          <div class="text-h5 q-mb-md">Unstaged</div>
          <div v-if="unstagedJson">
            <vue-json-pretty :data="unstagedJson" />
          </div>
        </template>

        <template v-slot:after>
          <div class="text-h5 q-mb-md">Staged</div>
          <div v-if="stagedJson">
            <vue-json-pretty :data="stagedJson" />
          </div>
        </template>
      </q-splitter>
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { invoke } from "@tauri-apps/api/tauri";
import * as git2rs from "../../api/git2rs";

export default defineComponent({
  components: {
    VueJsonPretty,
  },

  data() {
    return {
      response: null,
      stagedJson: null,
      unstagedJson: null,

      file: "",
      splitterModel: ref(50),
    };
  },

  mounted() {
    this.refreshStatus();
  },

  methods: {
    refreshStatus() {
      (async () => {
        this.stagedJson = (await this.getStatus("stage")) as any;
        this.unstagedJson = (await this.getStatus("workdir")) as any;
      })();
    },

    onReset() {
      this.file = "";
      this.response = null;
      this.refreshStatus();
    },

    onGitAdd() {
      var name = this.file;
      git2rs
        .add(name)
        .then((message) => {
          this.response = message as any;
        })
        .catch((e) => {
          if (e) {
            this.response = { error: JSON.stringify(e) } as any;
          }
        });
      this.refreshStatus();
    },

    onGitReset() {
      var name = this.file;
      git2rs
        .resetStage(name)
        .then((message) => {
          this.response = message as any;
        })
        .catch((e) => {
          if (e) {
            this.response = { error: JSON.stringify(e) } as any;
          }
        });
      this.refreshStatus();
    },

    async getStatus(args: string) {
      try {
        return await invoke("get_status", { statusType: args });
      } catch (e) {
        if (e) {
          return { error: JSON.stringify(e) };
        }
      }
    },
  },
});
</script>
