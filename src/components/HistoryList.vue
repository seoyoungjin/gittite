<template>
  <div class="q-pa-none" style="max-width: 350px">
    <!--
    <q-list dense bordered separator style="max-height: 600px" class="rounded-borders">
      <history-list-row v-for="item in logList" :item="item" :key="item.commit_id" />
    </q-list>
    -->

    <q-virtual-scroll
      style="max-height: 800px"
      :items="logList"
      separator
      bordered
      v-slot="{ item }"
    >
      <history-list-row :item="item" :key="item.commit_id" />
    </q-virtual-scroll>
  </div>
</template>

<script lang="ts">
import HistoryListRow from "@/components/HistoryListRow.vue";
import * as git2rs from "@/api/git2rs";

export default {
  name: "HistoryList",

  components: {
    HistoryListRow,
  },

  mounted() {
    this.getLogs();
  },

  data() {
    return {
      logList: null,
    };
  },

  methods: {
    getLogs() {
      (async () => {
        this.logList = await git2rs.getCommits();
      })();
    },
  },
};
</script>
