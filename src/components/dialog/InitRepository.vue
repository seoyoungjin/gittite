<template>
  <q-dialog ref="dialog" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Create a New Repository</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pt-none">
        <q-card-actions vertical>
          <q-form id="git-init">
            <q-input
              v-model="form.name"
              label="Name"
              placeholder="repository name"
            />
            <q-input v-model="form.directory" label="Local Path">
              <template v-slot:after>
                <q-btn no-caps @click="selectDirectory"> Choose... </q-btn>
              </template>
            </q-input>
            <!--
            <q-checkbox v-model="form.bareCheck" label="Bare" />
            <q-input
              v-model="form.templateDir"
              label="Template directory"
              :disable="!form.templateCheck"
            >
              <template v-slot:before>
                <q-checkbox v-model="form.templateCheck" />
              </template>
            </q-input>
            <q-input
              v-model="form.separateGitDir"
              label="Separate Git directory"
              :disable="!form.separateGitCheck"
            >
              <template v-slot:before>
                <q-checkbox v-model="form.separateGitCheck" />
              </template>
            </q-input>
            -->
          </q-form>
        </q-card-actions>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn color="primary" label="OK" @click="onOKClick" />
        <q-btn color="primary" label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import * as git2rs from "@/api/git2rs";
import { open } from "@tauri-apps/api/dialog";

export default {
  data() {
    return {
      form: {
        name: "",
        directory: "",
        bareCheck: false,
        templateCheck: false,
        templateDir: "",
        separateGitCheck: false,
        separateGitDir: "",
      },
    };
  },

  setup () {
    (async () => {
      let directory = await git2rs.get_param("cwd");
      alert(JSON.stringify(directory));
    })();
  },

  emits: [
    // REQUIRED
    "ok",
    "hide",
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

    onDialogHide() {
      this.$emit("hide");
    },

    onOKClick() {
      this.gitInit();

      this.$emit("ok");
      this.hide();
    },

    onCancelClick() {
      // we just need to hide the dialog
      this.hide();
    },

    async selectDirectory() {
      const selected = await open({
        directory: true,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      this.form.directory = selected;
    },

    gitInit: function () {
      // alert(JSON.stringify(this.form, null, 4));
      var dirname = this.form.directory + this.form.name;
      git2rs
        .init(dirname)
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
    },
  },
};
</script>
