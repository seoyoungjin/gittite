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
      v-if="item.tags && item.tags.length"
    >
      <q-badge color="grey" style="max-width: 100%">
        <div class="ellipsis">
          {{ item.tags[0].name }}
        </div>
        <q-icon name="arrow_right" v-if="item.tags.length > 1" />
      </q-badge>
    </q-item-section>
    <!-- popup -->
    <q-menu touch-position context-menu>
      <q-list dense style="min-width: 100px">
        <q-item clickable @click="onRevertChanges" v-close-popup>
          <q-item-section>Revert Changes in Commit...</q-item-section>
        </q-item>
        <q-separator />
        <q-item clickable @click="onCreateBranch" v-close-popup>
          <q-item-section>Create Branch from Commit</q-item-section>
        </q-item>
        <q-item clickable @click="onCreateTag" v-close-popup>
          <q-item-section>Create Tag...</q-item-section>
        </q-item>
        <q-separator />
        <q-item clickable @click="onCopySHA" v-close-popup>
          <q-item-section>Copy SHA</q-item-section>
        </q-item>
      </q-list>
    </q-menu>
  </q-item>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import RelativeTime from "@/components/RelativeTime.vue";
import { Commit } from "@/models/commit";

export default defineComponent({
  name: "HistoryListItem",

  props: {
    item: {
      type: Commit,
      required: true,
    },
  },

  components: {
    RelativeTime,
  },

  methods: {
    onRevertChanges(commit: Commit) {
      alert("onRevertChanges");
    },
    onCreateBranch(commit: Commit) {
      alert("onCreateBranch");
    },
    onCreateTag(commit: Commit) {
      alert("onCreateTag");
    },
    onCopySHA(commit: Commit) {
      alert("onCopySHA");
    },
  },
});
</script>
