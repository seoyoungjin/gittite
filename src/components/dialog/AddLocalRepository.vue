<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Add Local Repository</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pt-none">
        <q-card-actions vertical> </q-card-actions>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn color="primary" label="OK" @click="onOKClick" />
        <q-btn color="primary" label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import DialogMixin from "@/mixins/dialog";
import * as git2rs from "@/api/git2rs";

export default {
  name: "AddLocalRepository",
  mixins: [DialogMixin],

  data() {
    return {};
  },

  emits: [
    // REQUIRED
    "ok",
  ],

  methods: {
    // following method is REQUIRED
    show() {
      this.$refs.dialog.show();
    },

    // following method is REQUIRED
    hide() {
      this.$refs.dialog.hide();
    },

    onOKClick() {
      this.$emit("ok");
      this.hide();
    },

    onCancelClick() {
      this.hide();
    },
  },
};
</script>
