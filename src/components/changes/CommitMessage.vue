<template>
  <div>
    <q-card flat bordered class="q-px-xs q-pb-xs q-gutter-xs bg-grey-2">
      <q-input
        :dense="true"
        bg-color="white"
        v-model="commitMessageSummary"
        placeholder="Summary (required)"
        outlined
      >
        <template v-slot:before>
          <q-icon name="flight_takeoff" />
        </template>
      </q-input>

      <q-input
        v-model="commitMessageBody"
        placeholder="Description"
        type="textarea"
        bg-color="white"
        outlined
      />

      <div v-if="commitAmend">
        <div v-if="lastCommit">
          Your changes will modify your <strong>most recent commit</strong>.
        </div>
        <div v-else>No commit.</div>
      </div>

      <q-btn
        :disabled="!commitMessageSummary"
        color="primary"
        no-caps
        @click="gitCommitAmend()"
        v-if="commitAmend && lastCommit"
      >
        Amend last commit
      </q-btn>
      <q-btn
        :disabled="!(stagedFileLength > 0 && commitMessageSummary)"
        color="primary"
        no-caps
        @click="gitCommit()"
        v-else
      >
        Commit to&nbsp;<strong>{{ currentBranch }}</strong>
      </q-btn>
    </q-card>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import * as git2rs from "@/lib/git2rs";
import { useCommitStageStore } from "@/stores/commit-stage";
import { useHistoryStore } from "@/stores/history";
import { useRepositoryStore } from "@/stores/repository";
import type { CommitInfo } from "@/models/commit";

const initialData = () => ({
  commitMessageSummary: "",
  commitMessageBody: "",
  // amend
  lastCommit: null as CommitInfo | null,
});

export default defineComponent({
  name: "CommitMessage",

  props: {
    commitAmend: {
      type: Boolean,
      default: false,
    },
  },

  data() {
    return {
      ...initialData(),
    };
  },

  async mounted() {
    if (this.commitAmend) {
      this.lastCommit = await git2rs.commitInfo("HEAD").catch(() => {
        return null;
      });
      if (this.lastCommit) {
        this.commitMessageSummary = this.lastCommit.message.subject;
        this.commitMessageBody = this.lastCommit.message.body;
      }
    }
  },

  computed: {
    ...mapState(useCommitStageStore, ["stagedFileLength"]),
    ...mapState(useRepositoryStore, ["currentBranch"]),
  },

  methods: {
    ...mapActions(useCommitStageStore, ["updateCommitStage"]),
    ...mapActions(useHistoryStore, ["resetHistory", "loadCommitBatch"]),

    resetData() {
      // Object.assign(this.$data, this.$options.data.apply(this));
      const data = initialData();
      this.commitMessageSummary = data.commitMessageSummary;
      this.commitMessageBody = data.commitMessageBody;
    },

    async updateChangesAndHistory() {
      await this.updateCommitStage();
      this.resetHistory();
      await this.loadCommitBatch("HEAD", 0);
    },

    gitCommit() {
      var msg = this.commitMessageSummary;
      if (!msg) {
        alert("Enter commit message");
        return;
      }
      if (this.commitMessageBody) {
        msg = msg + "\n\n" + this.commitMessageBody;
      }
      git2rs
        .commit(msg)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
          this.resetData();
          this.updateChangesAndHistory();
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });
    },

    gitCommitAmend() {
      var msg = this.commitMessageSummary;
      if (!msg) {
        alert("Enter commit message");
        return;
      }
      if (this.commitMessageBody) {
        msg = msg + "\n\n" + this.commitMessageBody;
      }
      git2rs
        .commitAmend(msg)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
          this.resetData();
          this.updateChangesAndHistory();
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });
    },
  },
});
</script>
