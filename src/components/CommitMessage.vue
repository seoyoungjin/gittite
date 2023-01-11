<template>
  <div class="q-pa-xs q-gutter-xs bg-grey-2">
    <q-resize-observer @resize="onResize" />
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

    <q-btn
      :disabled="!(stagedFileLength > 0 && commitMessageSummary)"
      color="primary"
      no-caps
      @click="gitCommit()"
    >
      Commit to&nbsp;<strong>{{ currentBranch }}</strong>
    </q-btn>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import * as git2rs from "@/lib/git2rs";
import { useCommitStageStore } from "@/stores/commit-stage";
import { useRepositoryStore } from "@/stores/repository";

const initialData = () => ({
  commitMessageSummary: "",
  commitMessageBody: "",
});

export default defineComponent({
  name: "CommitMessage",
  data() {
    return {
      ...initialData(),
      clientHeight: 0,
    };
  },
  computed: {
    ...mapState(useCommitStageStore, ["stagedFileLength"]),
    ...mapState(useRepositoryStore, ["currentBranch"]),
  },
  methods: {
    resetData() {
      // Object.assign(this.$data, this.$options.data.apply(this));
      const data = initialData();
      // Object.keys(data).forEach((k) => (this[k] = data[k]));
      this.commitMessageSummary = data.commitMessageSummary;
      this.commitMessageBody = data.commitMessageBody;
    },
    onResize(size: any) {
      // alert(JSON.stringify(size));
      if (size.height !== this.clientHeight) {
        this.clientHeight = size.height;
        this.$emit("resize", size);
      }
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
          this.$emit("commit");
          this.resetData();
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
