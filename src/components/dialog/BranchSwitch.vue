<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="unsetModal">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Switch Branch</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section>
        <q-card-actions class="q-gutter-sm" vertical>
          <div>You have changes on this branch.</div>
          <q-list dense padding>
            <q-item v-for="item in items" :key="item.key">
              <q-item-section avatar>
                <q-radio v-model="switch_option" :val="item.key" />
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ item.title }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </q-card-actions>
      </q-card-section>

      <q-separator />

      <q-card-actions align="right">
        <q-btn
          no-caps
          color="primary"
          label="Switch Branch"
          @click="onOKClick"
        />
        <q-btn no-caps label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import ModalMixin from "@/mixins/modal";

export default defineComponent({
  name: "BranchSwitch",
  mixins: [ModalMixin],

  data() {
    return {
      items: [] as any[],
      switch_option: ref("stash"),
    };
  },

  emits: ["ok"],

  computed: {
    ...mapState(useRepositoryStore, ["currentBranch", "branchToSwitch"]),
  },

  methods: {
    show() {
      (this.$refs.dialog as any).show();
    },
    hide() {
      (this.$refs.dialog as any).hide();
    },
    onOKClick() {
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },

    // dialog specific
    onDialogShow() {
      this.setModal();
      this.items = [
        {
          title: `Stash changes on ${this.currentBranch}`,
          key: "stash",
        },
        {
          title: `Bring changes to ${this.branchToSwitch}`,
          key: "move",
        },
      ];
    },

    onSelectionChanged() {
      // TODO
      // this.switch_option
    },
  },
});
</script>
