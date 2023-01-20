<template>
  <div class="diff-options">
    <q-btn flat dense size="10pt">
      <q-icon name="o_settings" />
      <q-icon name="arrow_drop_down" />
    </q-btn>

    <q-popup-proxy>
      <div class="q-pa-md q-gutter-sm">
        <div class="text-weight-bold">White space</div>
        <q-checkbox
          dense
          size="sm"
          class="text-no-wrap"
          v-model="hideWhitespaceChanges"
          label="Hide white space change"
        />

        <div class="text-weight-bold">Diff display</div>
        <q-option-group
          dense
          size="sm"
          type="radio"
          class="q-ml-none"
          :options="options"
          v-model="display"
        />
      </div>
    </q-popup-proxy>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "DiffOptions",

  setup() {
    return {
      display: ref("unified"),

      options: [
        { label: "Unified", value: "unified" },
        { label: "Split", value: "split" },
      ],
    };
  },

  data() {
    return {
      hideWhitespaceChanges: false,
      showSideBySideDiff: false,
    };
  },

  methods: {},
});
</script>
