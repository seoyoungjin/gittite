<template>
  <q-dialog ref="dialog" @show="setModal" @hide="unsetModal">
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
import OctIcon from "@/components/OctIcon.vue";
import { mapState } from "pinia";
import { useCommitStageStore } from "@/stores/commit-stage";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  name: "BranchCreate",
  mixins: [ModalMixin],

  components: {
    OctIcon,
  },

  emits: ["ok"],

  computed: {
    ...mapState(useCommitStageStore, ["allStagedFiles", "allUnstagedFiles"]),

    allChanges() {
      // load from git2rs
      let merged = this.allUnstagedFiles.slice();
      this.allStagedFiles.forEach((k) => {
        if (merged.findIndex((i) => k.path === i.path) < 0) {
          merged.push(k);
        }
      });
      return merged;
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
      this.discardAllChanges();
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },
    // dialog specific
    discardAllChanges() {
      // TODO
    },
    discardChanges(item: StatusItem) {
      // TODO
      // discard dialog
      // if staged unstage first after then resetWorkdir
      git2rs
        .resetWorkdir(item.path)
        .then(async () => {
          await this.updateCommitStage();
        })
        .catch((e) => {
          this.showNotification(e.toString());
        });
    },
  },
});
</script>
