<template>
  <div class="q-pa-md" style="max-width: 350px">
    <q-list dense bordered padding class="rounded-borders">
      <q-item v-for="item in logData" clickable v-ripple>
        <q-item-section>
          {{ item.commit_id }} <br />
          {{ item.author }} <br />
          {{ item.summary }}
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import * as git2rs from "@/api/git2rs";

export default {
  mounted() {
    this.getLogs();
  },

  data() {
    return {
      logData: null,
    };
  },

  methods: {
    getLogs() {
      (async () => {
        this.logData = await git2rs.getCommits();
      })();
    },
  }
};
</script>
