<template>
  <div id="commit-summary" class="q-pa-xs">
    <q-bar dense class="bg-grey-1 text-no-wrap text-weight-bold">
      {{ historyMessageSubject }}
    </q-bar>

    <q-bar dense class="bg-grey-1 text-no-wrap" v-if="historyCommitInfo">
      <div>
        {{ historyCommitInfo.author.name }}
      </div>

      <div>
        <OctIcon symbol="gitCommit" size="14pt" />
        {{ historyCommitInfo.id.substring(0, 7) }}
        <q-tooltip class="text-subtitle2">
          {{ historyCommitInfo.id }}
        </q-tooltip>
      </div>

      <div>
        <OctIcon symbol="diff" size="14pt" />
        {{ historyCommitFiles.length }} changed files
      </div>

      <div class="text-green">+ TBD</div>

      <div class="text-red">- TBD</div>

      <div v-if="historyTagsLength" class="ellipsis">
        <OctIcon symbol="tag" size="14pt" />
        {{ historyTags.join(", ") }}
        <q-tooltip>
          <div v-html="historyTags.join('<br/>')" />
        </q-tooltip>
      </div>

      <DiffOptions />
    </q-bar>

    <q-separator color="grey-4" />

    <div
      class="col-auto bg-grey-2"
      style="white-space: pre-line"
      v-if="historyMessageBody"
    >
      {{ historyMessageBody }}
      <q-separator color="grey-4" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import { useHistoryStore } from "@/stores/history";
import DiffOptions from "@/components/diff/DiffOptions.vue";
import OctIcon from "@/components/OctIcon.vue";

export default defineComponent({
  name: "CommitSummary",

  components: {
    DiffOptions,
    OctIcon,
  },

  computed: {
    ...mapState(useHistoryStore, [
      "historyCurrent",
      "historyCommitInfo",
      "historyCommitFiles",
    ]),

    historyMessageSubject(): string {
      if (!this.historyCommitInfo) {
        return "";
      } else {
        return this.historyCommitInfo.message.subject;
      }
    },

    historyMessageBody(): string | null {
      if (!this.historyCommitInfo) {
        return null;
      } else {
        return this.historyCommitInfo.message.body;
      }
    },

    historyTagsLength(): number {
      if (!this.historyCurrent || !this.historyCurrent.tags) return 0;
      return this.historyCurrent.tags.length;
    },

    historyTags(): string[] {
      if (!this.historyCurrent || !this.historyCurrent.tags) return [];
      return this.historyCurrent.tags.map((e) => e.name);
    },
  },
});
</script>
