<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="unsetModal">
    <q-card class="q-dialog-plugin" v-if="!cloning">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Clone a Repository</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pt-none">
        <q-card-actions vertical>
          <q-input v-model="remoteUrl" label="Git URL" />
          <q-input v-model="repositoryName" label="Repository Name" />
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
          :disable="!(remoteUrl && repositoryName)"
        />
        <q-btn no-caps label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
    <q-card v-else class="q-dialog-plugin">
      <div class="q-pa-md">
        <h6>Cloning the repository</h6>
        <RemoteProgress
          ref="progressRef"
          message="start"
          @progressDone="progressDone"
        />
      </div>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapState } from "pinia";
import { useAppStore } from "@/stores/app";
import ModalMixin from "@/mixins/modal";
import { open } from "@tauri-apps/api/dialog";
import RemoteProgress from "@/components/RemoteProgress.vue";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  name: "CloneRepository",
  mixins: [ModalMixin],

  components: {
    RemoteProgress,
  },

  data() {
    return {
      remoteUrl: "",
      repositoryName: "",
      localPath: "",
      cloning: false,
    };
  },

  emits: ["ok"],

  computed: {
    ...mapState(useAppStore, ["CWD"]),
  },

  methods: {
    show() {
      (this.$refs.dialog as any).show();
    },

    hide() {
      (this.$refs.dialog as any).hide();
    },

    onOKClick() {
      this.cloneRepository();
      this.$emit("ok");
      // this.hide();
    },

    onCancelClick() {
      this.hide();
    },

    // dialog specific
    onDialogShow() {
      this.localPath = this.CWD;
      this.setModal();
    },

    async selectDirectory() {
      const selected = await open({
        directory: true,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      this.localPath = selected.split("\\").join("/");
    },

    async cloneRepository() {
      const repositoryPath = this.localPath + "/" + this.repositoryName;
      this.cloning = true;
      await git2rs
        .clone(this.remoteUrl, repositoryPath)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });
      this.cloning = false;

      // git2rs.testProgress().then(result => {
      //    this.cloning = false;
      // });
    },

    progressDone() {
      this.hide();
    },
  },
});
</script>
