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
export default {
  name: "CommitMessage",
  props: {
    branchName: {
      type: String,
      default: "",
    },
  },
  data() {
    return {
      commitMessageSummary: "",
    };
  },
  methods: {
    commitMessageButton() {},
  },
};
</script>
