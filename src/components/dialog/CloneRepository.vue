<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Clone a Repository</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pt-none">
        <q-card-actions vertical>
          <q-input v-model="gitURL" label="Git URL" />
          <q-input v-model="localPath" label="Local Path">
            <template v-slot:after>
              <q-btn no-caps @click="selectDirectory"> Choose... </q-btn>
            </template>
          </q-input>
        </q-card-actions>
      </q-card-section>

      <q-separator />

      <q-card-actions align="right">
        <q-btn
          color="primary"
          label="OK"
          @click="onOKClick"
          :disable="!gitURL"
        />
        <q-btn color="primary" label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import DialogMixin from "@/mixins/dialog";
import { usePropsStore } from "@/stores/props";
import * as git2rs from "@/api/git2rs";
import { sep } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";

export default {
  name: "CloneRepository",
  mixins: [DialogMixin],

  data() {
    const store = usePropsStore();
    return {
      gitURL: "",
      localPath: store.CWD,
    };
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

    async selectDirectory() {
      const selected = await open({
        directory: true,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      this.localPath = selected;
    },
  },
};
</script>
