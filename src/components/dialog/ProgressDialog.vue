<template>
  <q-dialog ref="dialog" @show="setModal" @hide="unsetModal">
    <q-card class="q-dialog-plugin">
      <!-- progress -->
      <q-card-section>
        <RemoteProgress ref="progressRef" />
      </q-card-section>

      <!-- buttons example -->
      <q-card-actions align="right">
        <q-btn color="primary" label="OK" @click="onOKClick" />
        <q-btn color="primary" label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ModalMixin from "@/mixins/modal";
import RemoteProgress from "@/components/RemoteProgress.vue";

export default defineComponent({
  mixins: [ModalMixin],
  components: {
    RemoteProgress,
  },

  data() {
    return {};
  },

  methods: {
    show() {
      (this.$refs.dialog as any).show();
    },
    hide() {
      (this.$refs.dialog as any).hide();
    },
    onOKClick() {
      this.startRemoteProgress();
      // this.hide();
    },
    onCancelClick() {
      this.hide();
    },
    // remote progress
    startRemoteProgress() {
      (this.$refs.progressRef as any).startProgress();
    },
  },
});
</script>
