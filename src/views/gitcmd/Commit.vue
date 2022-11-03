<template>
  <div class="q-pa-md">
    <div class="q-gutter-y-md">
      <q-card>
        <q-tabs
          v-model="tab"
          dense
          class="bg-grey-3"
          align="justify"
          narrow-indicator
        >
          <q-tab name="commit" icon="commit" label="Commit" />
          <q-tab name="amend" icon="edit" label="Amend" />
          <q-tab name="commit_info" icon="info" label="Commit Info" />
          <q-tab name="commit_files" icon="info" label="Commit Files" />
        </q-tabs>

        <q-separator />

        <q-tab-panels v-model="tab" animated>
          <q-tab-panel name="commit">
            <div class="text-h6">Commit</div>

            <q-form @submit="onCommit" class="q-gutter-md">
              <q-input
                v-model="commitForm.message"
                label="Enter commit message"
                filled
                type="textarea"
              />
              <div>
                <q-btn label="Submit" type="submit" color="primary" />
              </div>
            </q-form>
            <br />
            <br />
            <div>
              <vue-json-pretty :data="response" />
            </div>
            <br />
            <div class="text-h5 q-mb-md">Staged</div>
            <div>
              <vue-json-pretty :data="stagedJson" />
            </div>
          </q-tab-panel>

          <q-tab-panel name="amend">
            <div class="text-h6">Amend</div>

            <q-form @submit="onAmend" class="q-gutter-md">
              <q-input
                v-model="amendForm.message"
                label="Enter commit message"
                filled
                type="textarea"
              />
              <div>
                <q-btn label="Submit" type="submit" color="primary" />
              </div>
            </q-form>
            <br />
            <br />
            <div>
              <vue-json-pretty :data="response" />
            </div>
          </q-tab-panel>

          <q-tab-panel name="commit_info">
            <div class="text-h6">Commit Info</div>

            <q-form @submit="getCommitInfo" class="q-gutter-md">
              <q-input
                v-model="infoForm.commitId"
                label="Coomit ID"
                hint="Enter Commit ID"
              />
              <div>
                <q-btn label="Submit" type="submit" color="primary" />
              </div>
            </q-form>
            <br />
            <br />
            <div>
              <vue-json-pretty :data="response" />
            </div>
          </q-tab-panel>

          <q-tab-panel name="commit_files">
            <div class="text-h6">Commit Files</div>

            <q-form @submit="getCommitFiles" class="q-gutter-md">
              <q-input
                v-model="infoForm.commitId"
                label="Coomit ID"
                hint="Enter Commit ID"
              />
              <div>
                <q-btn label="Submit" type="submit" color="primary" />
              </div>
            </q-form>
            <pre>
Todo: diff --name-status
            </pre>
            <br />
            <br />
            <div>
              <vue-json-pretty :data="response" />
            </div>

          </q-tab-panel>
        </q-tab-panels>
      </q-card>
    </div>
  </div>
</template>

<script lang="ts">
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import * as git2rs from "../../api/git2rs";

export default {
  components: {
    VueJsonPretty,
  },

  setup() {
    return {
      tab: ref("commit"),
    };
  },

  mounted() {
    this.refresh();
  },

  data() {
    return {
      commitForm: {
        message: null,
      },
      amendForm: {
        // TODO retried message
        message: null,
      },
      infoForm: {
        commitId: null,
      },
      stagedJson: null,
      response: null,
    };
  },

  methods: {
    refresh() {
      (async () => {
        this.commitForm.message = null;
        this.amendForm.message = null;
        this.stagedJson = await git2rs.getStatus("stage");
        // alert(this.stagedJson);
      })();
    },

    onCommit() {
      if (Object.keys(this.stagedJson).length < 1) {
        alert("Nothing to commit");
        return;
      }
      const message = this.commitForm.message;
      invoke("commit", { args: message })
        .then((message) => {
          this.response = message;
        })
        .catch((e) => {
          if (typeof e == "string") {
            this.response = { error: e };
          } else {
            this.response = { error: JSON.stringify(e) };
          }
        });
      this.refresh();
    },

    onAmend() {
      const message = this.amendForm.message;
      invoke("amend", { args: message })
        .then((message) => {
          this.response = message;
        })
        .catch((e) => {
          if (typeof e == "string") {
            this.response = { error: e };
          } else {
            this.response = { error: JSON.stringify(e) };
          }
        });
      this.refresh();
    },

    getCommitInfo() {
      const commitId = this.infoForm.commitId;
      invoke("commit_info", { args: commitId })
        .then((message) => {
          this.response = message;
        })
        .catch((e) => {
          if (typeof e == "string") {
            this.response = { error: e };
          } else {
            this.response = { error: JSON.stringify(e) };
          }
        });
    },

    getCommitFiles() {
      const commitId = this.infoForm.commitId;
      invoke("commit_files", { args: commitId })
        .then((message) => {
          this.response = message;
        })
        .catch((e) => {
          if (typeof e == "string") {
            this.response = { error: e };
          } else {
            this.response = { error: JSON.stringify(e) };
          }
        });
    },
  },
};
</script>
