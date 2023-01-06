<template>
  <q-item
    class="item-row"
    @mouseover="showOptions = true"
    @mouseout="showOptions = false"
  >
    <!--Main Part-->
    <q-item-section>
      <q-item-label class="text-subtitle2">{{ item.summary }}</q-item-label>
      <q-item-label caption>
        {{ item.author }} -
        <RelativeTime :date="new Date(item.time * 1000)" />
      </q-item-label>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import RelativeTime from "./RelativeTime.vue";

export default defineComponent({
  name: "HistoryListRow",
  props: ["item"],
  components: {
    RelativeTime,
  },
  data() {
    return {
      showOptions: false,
      showPopover: false,
    };
  },
});
</script>
