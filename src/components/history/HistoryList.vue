<template>
  <div class="q-pa-none" style="width: 100%">
    <q-virtual-scroll
      :items="commitLogs"
      bordered
      separator
      v-slot="{ item, index }"
    >
      <HistoryListItem
        :key="index"
        :item="item"
        dense
        clickable
        @click="clickItem(item)"
      />
    </q-virtual-scroll>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import HistoryListItem from "./HistoryListItem.vue";

export default defineComponent({
  name: "HistoryList",

  components: {
    HistoryListItem,
  },

  computed: {
    ...mapState(useRepositoryStore, ["commitLogs"]),
  },

  methods: {
    clickItem(item: any) {
      // alert(JSON.stringify(item, null, 4));
      this.$emit("selectItem", item);
    },
  },
});
</script>
