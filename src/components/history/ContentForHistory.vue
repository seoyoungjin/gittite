<template>
  <div class="column" style="height: 100%">
    <CommitSummary />
    <div class="col">
      <q-splitter
        v-model="splitterModel"
        :limits="[100, 300]"
        unit="px"
        before-class="overflow-hidden"
        after-class="overflow-hidden"
        style="height: 100%"
      >
        <template v-slot:before>
          <CommitFileList @selectItem="handleSelectItem" />
        </template>

        <template v-slot:after>
          <DiffView :selection="historyCurrent" :selectedFile="selectedFile" />
        </template>
      </q-splitter>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { mapState } from "pinia";
import { useHistoryStore } from "@/stores/history";
import CommitSummary from "./CommitSummary.vue";
import CommitFileList from "./CommitFileList.vue";
import DiffView from "@/components/DiffView.vue";
import type { StatusItem } from "@/models/status";

export default defineComponent({
  name: "ContentForHistory",

  setup() {
    return {
      splitterModel: ref(250),
    };
  },

  components: {
    CommitSummary,
    CommitFileList,
    DiffView,
  },

  computed: {
    ...mapState(useHistoryStore, ["historyCurrent", "historyCommitFiles"]),

    selectedFile(): string {
      // console.log(this.historyCommitFiles.includes(this.selectedItem));
      if (this.historyCommitFiles.includes(this.selectedItem as any)) {
        if (this.selectedItem) return this.selectedItem.path;
      } else if (this.historyCommitFiles.length) {
        return this.historyCommitFiles[0].path;
      }
      return "";
    },
  },

  data() {
    return {
      selectedItem: null as StatusItem | null,
    };
  },

  methods: {
    handleSelectItem(item: any) {
      this.selectedItem = item;
    },
  },
});
</script>
