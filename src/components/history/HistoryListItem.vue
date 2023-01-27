<template>
  <q-item class="q-px-xs">
    <!--main -->
    <q-item-section>
      <q-item-label class="text-subtitle2 ellipsis">{{
        item.summary
      }}</q-item-label>
      <q-item-label caption>
        {{ item.author }} -
        <RelativeTime :date="new Date(item.time * 1000)" />
      </q-item-label>
    </q-item-section>
    <!-- tag -->
    <q-item-section
      side
      class="no-padding"
      style="max-width: 40%"
      v-if="item.tags.length"
    >
      <q-badge color="grey" style="max-width: 100%">
        <div class="ellipsis">
          {{ item.tags[0].name }}
        </div>
        <q-icon name="arrow_right" v-if="item.tags.length > 1" />
      </q-badge>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import RelativeTime from "@/components/RelativeTime.vue";

export default defineComponent({
  name: "HistoryListItem",
  props: ["item"],
  components: {
    RelativeTime,
  },
});
</script>
