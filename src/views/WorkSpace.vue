<template>
  <q-layout view="hHh lpR fFf" class="bg-grey-1">
    <q-splitter
      v-model="splitterModel"
      :limits="[300, 600]"
      unit="px"
      before-class="overflow-hidden"
      after-class="overflow-hidden"
      style="height: 100vh"
    >
      <template v-slot:before>
        <ToolBar />
        <q-tabs
          v-model="tab"
          dense
          no-caps
          class="bg-orange text-black shadow-2"
        >
          <q-tab name="changes" label="Changes" />
          <q-tab name="history" label="History" />
        </q-tabs>

        <q-separator />

        <q-tab-panels v-model="tab" animated class="fit">
          <q-tab-panel name="changes" class="q-pa-sm">
            <changes-list v-on:selectItem="handleSelectItem" />
          </q-tab-panel>

          <q-tab-panel name="history" class="q-pa-sm">
            <history-list v-on:selectItem="handleSelectItem" />
          </q-tab-panel>
        </q-tab-panels>
      </template>

      <template v-slot:after>
        <ToolBar2 />
        <q-scroll-area class="fit">
          <div class="q-pa-md">
            <diff-view :curSelected="curSelected" />
          </div>
        </q-scroll-area>
      </template>
    </q-splitter>
  </q-layout>
</template>

<script lang="ts">
import { ref } from "vue";
import ToolBar from "@/layouts/ToolBar.vue";
import ToolBar2 from "@/layouts/ToolBar2.vue";
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
    ToolBar,
    ToolBar2,
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