<template>
  <div>
    <q-item>
      <q-item-section>
        <q-item-label caption>
          Committed <RelativeTime :date="authorDate" />
        </q-item-label>
        <q-item-label>
          Summary
        </q-item-label>
      </q-item-section>
      <q-item-section side>
        <q-btn no-caps color="primary" :disable="aa" @click="undoLastCommit()">
          Undo
        </q-btn>
      </q-item-section>
    </q-item>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  name: "UndoCommit",

  data() {
    return {
      authorDate: null as Date | null,
      commitMessageSummary: "",
    };
  },

  async mounted() {
    let lastCommit = await git2rs.commitInfo("HEAD").catch(() => {
      return null;
    });
    if (lastCommit) {
      this.commitMessageSummary = lastCommit.message.subject;
      this.authoData = lastCommit.author.time;
    }
  },

  methods: {
    undoLastCommit() {},
  },
});
</script>
