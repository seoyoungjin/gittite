<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Discard All Changes</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section>
        <q-icon :name="octAlert16" size="16pt" color="yellow-7" />
        Are you sure you want to discard all changes?
      </q-card-section>

      <q-separator />

      <q-card-actions align="right">
        <q-btn no-caps color="primary" label="OK" @click="onOKClick" />
        <q-btn no-caps label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import DialogMixin from "@/mixins/dialog";
import { octAlert16 } from "quasar-extras-svg-icons/oct-icons-v17";

export default defineComponent({
  name: "BranchCreate",
  mixins: [DialogMixin],

  data() {
    return {
      octAlert16,
      branchName: "",
    };
  },

  emits: ["ok"],

  computed: {
    disableOkButton() {
      return !this.branchName;
    },
  },

  methods: {
    show() {
      (this.$refs.dialog as any).show();
    },
    hide() {
      (this.$refs.dialog as any).hide();
    },
    onOKClick() {
      // TODO
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },

    // dialog specific
  },
});
</script>
