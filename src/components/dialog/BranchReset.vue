<template>
  <q-dialog ref="dialog" @show="setModal" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Discard All Changes?</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section>
        <OctIcon symbol="alert" size="14pt" color="yellow-7" />
        Are you sure you want to discard all changes?
      </q-card-section>

      <q-card-section>
        <q-list dense>
          <q-item v-for="item in allChanges" :key="item.path">
            <q-item-label>{{ item.path }}</q-item-label>
          </q-item>
        </q-list>
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
import ModalMixin from "@/mixins/modal";
import { mapActions, mapState } from "pinia";
import { useAppStore } from "@/stores/app";
import { useCommitStageStore } from "@/stores/commit-stage";
import OctIcon from "@/components/OctIcon.vue";
import type { StatusItem } from "@/models/status";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  name: "BranchReset",
  mixins: [ModalMixin],

  components: {
    OctIcon,
  },

  emits: ["ok"],

  computed: {
    ...mapState(useCommitStageStore, ["allStagedFiles", "allUnstagedFiles"]),
    ...mapState(useAppStore, ["DiscardItem"]),

    allChanges() {
      let merged: StatusItem[] = [];
      if (this.DiscardItem) {
        merged.push(this.DiscardItem);
      } else {
        merged = this.allUnstagedFiles.slice();
        this.allStagedFiles.forEach((k) => {
          if (merged.findIndex((i) => k.path === i.path) < 0) {
            merged.push(k);
          }
        });
      }
      return merged;
    },
    // async allChanges() {
    //   let all_changed = await git2rs.getStatus("all").catch(() => {
    //     return [];
    //   });
    //   return all_changed;
    // },
  },

  methods: {
    ...mapActions(useCommitStageStore, [
      "discardAllChanges",
      "updateCommitStage",
    ]),
    ...mapActions(useAppStore, ["setDiscardItem"]),

    show() {
      (this.$refs.dialog as any).show();
    },
    hide() {
      (this.$refs.dialog as any).hide();
    },
    onOKClick() {
      if (this.DiscardItem) {
        this.discardChanges(this.DiscardItem);
      } else {
        this.discardAllChanges();
      }
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },
    // dialog specific
    onDialogHide() {
      this.setDiscardItem(null);
      this.unsetModal();
    },
    async discardChanges(item: StatusItem) {
      git2rs
        .resetWorkdir(item.path)
        .then(async () => {
          await this.updateCommitStage();
        })
        .catch(() => {});
    },
  },
});
</script>
