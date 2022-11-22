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
            <changes-list v-on:selectItem="handleSelectItem" />
          </q-tab-panel>

          <q-tab-panel name="history">
            <history-list v-on:selectItem="handleSelectItem" />
          </q-tab-panel>
        </q-tab-panels>
      </template>

      <template v-slot:after>
        <div class="q-pa-md">
          <diff-view :curSelected="curSelected" />
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
      curSelected: Object,
    };
  },

  components: {
    ChangesList,
    HistoryList,
    DiffView,
  },

  methods: {
    handleSelectItem(item: any) {
      this.curSelected = item;
    },
  },
};
</script>
