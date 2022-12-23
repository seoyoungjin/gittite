<template>
  <div class="q-pa-none" style="max-width: 80vh">
    <q-list
      dense
      bordered
      separator
      style="max-height: 80vh"
      class="rounded-borders"
    >
      <history-list-row
        v-for="item in logList"
        :item="item"
        :key="item.commit_id"
        clickable
        @click="clickItem(item)"
      />
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import HistoryListRow from "./HistoryListRow.vue";
import * as git2rs from "@/api/git2rs";
import type { CommitData } from "@/api/types";

export default defineComponent({
  name: "HistoryList",

  components: {
    HistoryListRow,
  },

  mounted() {
    this.getLogs();
  },

  data() {
    return {
      logList: [] as CommitData[],
    };
  },

  methods: {
    getLogs() {
      (async () => {
        this.logList = await git2rs.getCommits();
      })();
    },

    clickItem(item: any) {
      // alert(JSON.stringify(item, null, 4));
      this.$emit("selectItem", item);
    },
  },
});
</script>
