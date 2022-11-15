<template>
  <div class="q-pa-md" style="max-width: 350px">
    <q-list dense bordered padding class="rounded-borders">
      <q-item clickable v-ripple>
        <q-item-section>
          Unstaged
        </q-item-section>
      </q-item>

      <q-item v-for="item in unstagedData" clickable v-ripple>
        <q-item-section>
          {{ item.status }} {{ item.path }}
        </q-item-section>
      </q-item>
    </q-list>

    <q-list dense bordered padding class="rounded-borders">
      <q-item clickable v-ripple>
        <q-item-section>
          Staged
        </q-item-section>
      </q-item>

      <q-item v-for="item in stagedData" clickable v-ripple>
        <q-item-section>
          {{ item.status }} {{ item.path }}
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import * as git2rs from "@/api/git2rs";

export default {
  mounted() {
    this.refreshStatus();
  },

  data() {
    return {
      stagedData: null,
      unstagedData: null,
    };
  },

  methods: {
    refreshStatus() {
      (async () => {
        this.stagedData = await git2rs.getStatus("stage");
        this.unstagedData = await git2rs.getStatus("workdir");
      })();
    },
  }
};
</script>
