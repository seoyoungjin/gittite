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
            <div v-if="response">
              <vue-json-pretty :data="response" />
            </div>
            <br />
            <div class="text-h5 q-mb-md">Staged</div>
            <div v-if="stagedJson">
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
            <div v-if="response">
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
            <div v-if="response">
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
            <br />
            <br />
            <div v-if="response">
              <vue-json-pretty :data="response" />
            </div>
          </q-tab-panel>
        </q-tab-panels>
      </q-card>
    </div>
  </div>
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
        message: "",
      },
      amendForm: {
        message: "",
      },
      infoForm: {
        commitId: "",
      },
      stagedJson: [],
      response: null,
    };
  },

  methods: {
    refresh() {
      (async () => {
        let lastCommitInfo = await git2rs.commitInfo("HEAD");
        this.commitForm.message = "";
        // TODO message format
        this.amendForm.message = lastCommitInfo.message.subject;
        if (lastCommitInfo.message.body) {
          this.amendForm.message =
            lastCommitInfo.message.subject +
            "\n\n" +
            lastCommitInfo.message.body;
        }
        this.stagedJson = (await git2rs.getStatus("stage")) as any;
      })();
    },

    async onCommit() {
      if (Object.keys(this.stagedJson).length < 1) {
        alert("Nothing to commit");
        return;
      }
      const message = this.commitForm.message;
      this.response = (await git2rs.commit(message).catch((e) => {
        if (e) {
          this.response = { error: JSON.stringify(e) } as any;
        }
      })) as any;
      this.refresh();
    },

    async onAmend() {
      const message = this.amendForm.message;
      this.response = (await git2rs.commitAmend(message).catch((e) => {
        this.response = { error: JSON.stringify(e) } as any;
      })) as any;
      this.refresh();
    },

    async getCommitInfo() {
      const commitId = this.infoForm.commitId;
      this.response = (await git2rs.commitInfo(commitId).catch((e) => {
        this.response = { error: JSON.stringify(e) } as any;
      })) as any;
    },

    async getCommitFiles() {
      const commitId = this.infoForm.commitId;
      this.response = (await git2rs.commitFiles(commitId).catch((e) => {
        this.response = { error: JSON.stringify(e) } as any;
      })) as any;
    },
  },
});
</script>
