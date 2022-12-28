<template>
  <div class="q-pa-none" style="max-width: 80vh">
    <q-list
      dense
      bordered
      separator
      style="max-height: 80vh"
      class="rounded-borders"
    >
      <history-list-row
        v-for="item in commitLogs"
        :item="item"
        :key="item.commit_id"
        clickable
        @click="clickItem(item)"
      />
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import HistoryListRow from "./HistoryListRow.vue";

export default defineComponent({
  name: "HistoryList",

  components: {
    HistoryListRow,
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
