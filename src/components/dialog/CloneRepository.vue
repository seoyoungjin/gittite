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
          no-caps
          color="primary"
          label="OK"
          @click="onOKClick"
          :disable="!gitURL"
        />
        <q-btn no-caps label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { usePropStore } from "@/stores/props";
import DialogMixin from "@/mixins/dialog";
import * as git2rs from "@/api/git2rs";
import { open } from "@tauri-apps/api/dialog";

export default defineComponent({
  name: "CloneRepository",
  mixins: [DialogMixin],

  data() {
    const store = usePropStore();
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
});
</script>
