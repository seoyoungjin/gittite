<template>
  <q-item
    class="item-row"
    @mouseover.native="showOptions = true"
    @mouseout.native="showOptions = false"
  >

    <!--Main Part-->
    <q-item-section>
      <q-item-label>{{ item.summary }}</q-item-label>
      <q-item-label caption>{{ item.author }} - {{ item.date }}</q-item-label>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { mapActions, mapState } from "pinia";
import { format } from "quasar";

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
    showCheckbox() {
      return false;
    },
  },
  methods: {
  },
};
</script>
