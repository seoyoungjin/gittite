<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Add Local Repository</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pa-sm">
        <q-card-actions vertical>
          Local Path
          <q-input v-model="localPath" dense placeholder="repository path">
            <template v-slot:after>
              <q-btn no-caps @click="selectDirectory"> Choose... </q-btn>
            </template>
          </q-input>
        </q-card-actions>
      </q-card-section>

      <!-- TODO v-if -->
      <q-card-section class="row" v-if="Error">
        <q-icon :name="octAlert16" size="16pt" color="yellow-7" />
        This directory does not appear to be a Git repository.
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
import DialogMixin from "@/mixins/dialog";
import { octAlert16 } from "quasar-extras-svg-icons/oct-icons-v17";
import { useSettingsStore } from "@/stores/settings";

export default {
  name: "AddLocalRepository",
  mixins: [DialogMixin],

  data() {
    const store = useSettingsStore();
    return {
      octAlert16,
      localPath: "",
      store,
    };
  },

  emits: [
    // REQUIRED
    "ok",
  ],

  methods: {
    show() {
      this.$refs.dialog.show();
    },
    hide() {
      this.$refs.dialog.hide();
    },
    onOKClick() {
      this.addLocalRepository(this.localPath);
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },

    // user method
    addLocalRepository(path: string) {
      // alert(path);
      if (path !== "") {
        this.store.addRepository(path);
      }
    },
    // TODO watch: {path: isGit},
  },
};
</script>
