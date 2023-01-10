<template>
  <div class="column" style="height: 100%">
    <div class="col-auto">
      commit summary and info
    </div>
    <div class="col-auto">
      commit message body
    </div>
    <div class="col">
      <q-splitter
        v-model="splitterModel"
        :limits="[100, 300]"
        unit="px"
        before-class="overflow-hidden"
        after-class="overflow-hidden"
        style="height: 100%"
      >
        <template v-slot:before>
          <q-list dense bordered padding>
            <q-item clickable v-ripple>
              <q-item-section>
                Item
              </q-item-section>
            </q-item>
          </q-list>
        </template>

        <template v-slot:after>
          <diff-view :selection="selection" />
        </template>
      </q-splitter>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import DiffView from "@/components/DiffView.vue";

export default defineComponent({
  name: "ContentForHistory",

  setup() {
    return {
      splitterModel: ref(250),
    };
  },

  props: {
    selection: {},
  },

  components: {
    DiffView,
  },
});
</script>