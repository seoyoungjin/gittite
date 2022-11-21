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
      :disabled="!stagedFileLength > 0"
      color="primary"
      @click="commitMessageButton()"
    >
      Commit to <strong>{{ branchName }}</strong>
    </q-btn>
  </div>
</template>

<script lang="ts">
import { mapState } from "pinia";
import { useCommitStageStore } from "@/stores/commitStage";
import * as git2rs from "@/api/git2rs";

export default {
  name: "CommitMessage",
  props: {
    branchName: {
      type: String,
      default: "",
    },
  },
  computed: {
    ...mapState(useCommitStageStore, ["stagedFileLength"]),
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
      if (this.commitMessageBody)
        msg = msg + "\n\n" + this.commitMessageBody;

      git2rs.commit(msg).catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            error: error,
          });
        });
    },
  },
};
</script>
