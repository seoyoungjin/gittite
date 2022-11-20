<template>
  <div>
    <q-splitter v-model="splitterModel" unit="px">
      <template v-slot:before>
        <q-tabs v-model="tab" no-caps class="bg-orange text-black shadow-2">
          <q-tab name="changes" label="Changes" />
          <q-tab name="history" label="History" />
        </q-tabs>

        <q-separator />

        <q-tab-panels v-model="tab" animated>
          <q-tab-panel name="changes">
            <ChangesList v-on:selectFile="handleSelectFile" />
          </q-tab-panel>

          <q-tab-panel name="history">
            <div class="text-h6">History</div>
            <HistoryList />
          </q-tab-panel>
        </q-tab-panels>
      </template>

      <template v-slot:after>
        <div class="q-pa-md">
          <DiffView :curSelected="curSelected" />
        </div>
      </template>
    </q-splitter>
  </div>
</template>

<script lang="ts">
import { ref } from "vue";
import ChangesList from "@/components/ChangesList.vue";
import HistoryList from "@/components/HistoryList.vue";
import DiffView from "@/components/DiffView.vue";

export default {
  setup() {
    return {
      splitterModel: ref(250),
      tab: ref("changes"),
    };
  },

  data() {
    return {
      curSelected: null,
    };
  },

  components: {
    ChangesList,
    HistoryList,
    DiffView,
  },

  methods: {
    handleSelectFile(item) {
      this.curSelected = item;
    },
  },
};
</script>
