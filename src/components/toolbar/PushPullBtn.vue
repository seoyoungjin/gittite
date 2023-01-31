<template>
  <q-btn flat no-caps :loading="loading" @click="startFetch">
    <q-icon left name="sync" />
    <q-item-section align="left">
      <q-item-label> Fetch </q-item-label>
      <q-item-label>
        <small class="text-no-wrap" v-if="lastFetched">
          Last fetched <RelativeTime :date="lastFetched" />
        </small>
        <small v-else>Never fetched</small>
      </q-item-label>
    </q-item-section>

    <template v-slot:loading>
      <q-spinner class="on-left" />
      Fetching
    </template>
  </q-btn>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import RelativeTime from "@/components/RelativeTime.vue";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  components: {
    RelativeTime,
  },

  data() {
    return {
      loading: false,
      lastFetched: null as any,
    };
  },

  async mounted() {
    const lastFetched: any = await git2rs.get_prop("LastFetchedTime");
    this.lastFetched = new Date(lastFetched * 1000);
  },

  methods: {
    async startFetch() {
      this.loading = true;
      // setTimeout(() => {
      //   loading.value = false;
      // }, 3000);
      await git2rs.fetchAll();
      this.lastFetched = new Date(Date.now());
      this.loading = false;
    },
  },
});
</script>
