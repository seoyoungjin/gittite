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

      <!-- error message -->
      <q-card-section class="row" v-if="localPath && errorCode">
        <q-icon :name="octAlert16" size="16pt" color="yellow-7" />
        {{ errorMessage }}
      </q-card-section>

      <q-separator />

      <q-card-actions align="right">
        <q-btn
          no-caps
          color="primary"
          label="OK"
          @click="onOKClick"
          :disable="disableOkButton"
        />
        <q-btn no-caps label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import DialogMixin from "@/mixins/dialog";
import { octAlert16 } from "quasar-extras-svg-icons/oct-icons-v17";
import { useSettingsStore } from "@/stores/settings";
import { open } from "@tauri-apps/api/dialog";
import * as git2rs from "@/api/git2rs";

enum Error {
  Ok = 0,
  NotGitRepository = 1,
  AlreadyExist = 2,
}

export default defineComponent({
  name: "AddLocalRepository",
  mixins: [DialogMixin],

  data() {
    const store = useSettingsStore();
    return {
      store,
      octAlert16,
      localPath: "",
      errorCode: Error.Ok,
    };
  },

  emits: ["ok"],

  methods: {
    show() {
      (this.$refs.dialog as any).show();
    },
    hide() {
      (this.$refs.dialog as any).hide();
    },
    onOKClick() {
      this.addLocalRepository(this.localPath);
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },

    // dialog specific
    async selectDirectory() {
      const selected = await open({
        directory: true,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      this.localPath = selected;
    },

    async addLocalRepository(path: string) {
      const is_git = await git2rs.isGitRepository(path);
      if (!is_git) {
        alert(path + " is not repository");
        return;
      }
      this.store.addRepository(path);
    },
  },

  computed: {
    disableOkButton() {
      return !(this.localPath && this.errorCode == Error.Ok);
    },
    errorMessage() {
      if (this.errorCode === Error.NotGitRepository) {
        return "This directory does not appear to be a Git repository.";
      } else if (this.errorCode === Error.AlreadyExist) {
        return "This directory is already in repository list.";
      }
      return "";
    },
  },

  watch: {
    localPath: async function (val: string) {
      if (!val) {
        return;
      }
      if (this.store.allRepository.includes(val)) {
        this.errorCode = Error.AlreadyExist;
        return;
      }
      let res = await git2rs.isGitRepository(val);
      if (res) {
        this.errorCode = Error.Ok;
      } else {
        this.errorCode = Error.NotGitRepository;
      }
    },
  },
});
</script>
