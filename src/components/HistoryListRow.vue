<template>
  <q-item
    class="item-row"
    @mouseover.native="showOptions = true"
    @mouseout.native="showOptions = false"
  >
    <!--Main Part-->
    <q-item-section>
      <q-item-label>{{ item.summary }}</q-item-label>
      <q-item-label caption>
        {{ item.author }} -
        {{ formattedString }}
      </q-item-label>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { mapActions, mapState } from "pinia";
import { date } from "quasar";

export default {
  name: "HistoryListRow",
  props: ["item"],
  components: {},
  data() {
    return {
      showOptions: false,
      showPopover: false,
    };
  },
  computed: {
    formattedString() {
      return date.formatDate(this.item.time * 1000, "YYYY-MM-DDTHH:mm:ss Z");
    },
  },
  methods: {},
};
</script>
