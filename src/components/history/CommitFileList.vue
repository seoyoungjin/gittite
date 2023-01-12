<template>
  <div class="q-pa-none" style="height: 100%">
    <q-virtual-scroll
      :items="commitFiles"
      bordered
      separator
      class="fit"
      v-slot="{ item, index }"
    >
      <q-item
        class="q-pa-none"
        :key="index"
        dense
        clickable
        @click="clickItem(item)"
      >
        <q-item-section side class="q-pa-xs">
          <OctStatusIcon v-bind:status="item.status" size="14pt" />
        </q-item-section>
        <q-item-section>
          <q-item-label class="ellipsis">{{ item.path }}</q-item-label>
        </q-item-section>
      </q-item>
    </q-virtual-scroll>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import OctStatusIcon from "@/components/OctStatusIcon.vue";

export default defineComponent({
  props: {
    commitFiles: [] as any[],
  },

  components: {
    OctStatusIcon,
  },

  methods: {
    clickItem(item: any) {
      this.$emit("selectItem", item);
    },
  },
});
</script>
