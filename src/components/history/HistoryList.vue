<template>
  <div class="q-pa-none" style="height: 100%">
    <q-infinite-scroll ref="historyList" bordered separator @load="onLoad" :offset="200">
      <q-list dense bordered separator>
        <HistoryListItem
          v-for="(item, index) in commitLogs"
          :key="index"
          :item="item"
          dense
          clickable
          @click="clickItem(item)"
        />
      </q-list>
      <template v-slot:loading>
        <div class="row justify-center q-my-md">
          <q-spinner-dots color="primary" size="40px" />
        </div>
      </template>
    </q-infinite-scroll>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import HistoryListItem from "./HistoryListItem.vue";

export default defineComponent({
  name: "HistoryList",

  components: {
    HistoryListItem,
  },

  data() {
    return {
      historyLoadCompleted: false
    };
  },

  computed: {
    ...mapState(useRepositoryStore, ["commitLogs"]),
  },

  watch: {
    commitLogs(value: []) {
      // console.log("watch commitLogs", value.length, this.historyLoadCompleted);
      if (value.length == 0 && this.historyLoadCompleted) {
        (this.$refs.historyList as any).resume();
        this.historyLoadCompleted = false;
      }
    },
  },

  methods: {
    ...mapActions(useRepositoryStore, ["loadNextCommitBatch"]),

    clickItem(item: any) {
      this.$emit("selectItem", item);
    },

    onLoad(index: number, done: (stop: any) => void) {
      // stop, resume
      setTimeout(async () => {
        let commits = await this.loadNextCommitBatch();
        this.historyLoadCompleted = !commits || commits.length == 0;
        done(this.historyLoadCompleted);
      }, 500);
    },
  },
});
</script>
