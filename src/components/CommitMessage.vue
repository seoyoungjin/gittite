<template>
  <div class="q-pa-none q-gutter-md">
    <q-input
      :dense="true"
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
      outlined
    />

    <q-btn
      :disabled="!(stagedFileLength > 0 && commitMessageSummary)"
      color="primary"
      no-caps
      @click="commitMessageButton()"
    >
      Commit to&nbsp;<strong>{{ currentBranch }}</strong>
    </q-btn>
  </div>
</template>

<script lang="ts">
import { mapState } from "pinia";
import * as git2rs from "@/api/git2rs";
import { useCommitStageStore } from "@/stores/commitStage";
import { useRepositoryStore } from "@/stores/repository";

export default {
  name: "CommitMessage",
  computed: {
    ...mapState(useCommitStageStore, ["stagedFileLength"]),
    ...mapState(useRepositoryStore, ["currentBranch"]),
  },
  data() {
    return {
      commitMessageSummary: "",
      commitMessageBody: "",
    };
  },
  methods: {
    commitMessageButton() {
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
          this.$emit("commit");
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
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
};
</script>
