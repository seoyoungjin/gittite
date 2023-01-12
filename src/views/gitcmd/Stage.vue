<template>
  <q-page class="q-ma-lg">
    <h6>Stage Stage/Unstage</h6>

    <q-card>
      <q-card-section>
        <q-form @reset="onReset" class="q-gutter-md">
          <q-checkbox v-model="isDeleted" label="Check for Deleted File" />
          <q-input
            v-model="file"
            label="File to stage"
            hint="Enter file name"
          />

          <q-btn label="Stage" @click="onGitStage" color="primary" />
          <q-btn label="Unstage" @click="onGitUnstage" color="primary" />
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
import * as git2rs from "../../lib/git2rs";

export default defineComponent({
  components: {
    VueJsonPretty,
  },

  data() {
    return {
      splitterModel: ref(50),

      response: null,
      stagedJson: null,
      unstagedJson: null,

      file: "",
      isDeleted: false,
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

    onGitStage() {
      var name = this.file;
      let stage_function: (path: string) => Promise<boolean>;

      if (this.isDeleted == true) {
        stage_function = git2rs.stageRemovePath;
      } else {
        stage_function = git2rs.stageAddPath;
      }
      stage_function(name)
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

    onGitUnstage() {
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
        return await git2rs.getStatus(args);
      } catch (e) {
        if (e) {
          return { error: JSON.stringify(e) };
        }
      }
    },
  },
});
</script>
