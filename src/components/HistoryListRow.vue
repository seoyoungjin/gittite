<template>
  <q-item
    class="item-row"
    @mouseover.native="showOptions = true"
    @mouseout.native="showOptions = false"
    @click.native="clickItem(item)"
  >
    <!--Left Part Checkbox-->
    <q-item-side v-if="showCheckbox" class="text-center">
      <q-checkbox color="grey-6" v-model="selected" />
    </q-item-side>

    <!--Main Part-->
    <q-item-main>
      <q-item-tile label>{{ item.summary }}</q-item-tile>
    </q-item-main>

    <q-item-main>
      {{ item.author }}
    </q-item-main>

    <q-item-main>
        {{ item.commit_id }}
    </q-item-main>
  </q-item>
</template>

<script>
import { mapActions, mapGetters } from "pinia";
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

<!--
<style lang="stylus" scoped>
.item-row
    height: 45px
</style>
-->
