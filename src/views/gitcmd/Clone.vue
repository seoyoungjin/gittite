<template>
  <q-page class="q-ma-lg">
    <h6>Clone</h6>

    <q-card>
      <q-card-section>
        <q-form @submit="onSubmit" @reset="onReset" class="q-gutter-md">
          <q-input v-model="form.gitURL" label="gitURL" hint="Enter git url" />
          <q-input
            v-model="form.directory"
            label="Directory"
            hint="Enter directory"
          />

          <div>
            <q-btn label="Submit" type="submit" color="primary" />
            <q-btn
              label="Reset"
              type="reset"
              color="primary"
              flat
              class="q-ml-sm"
            />
          </div>
        </q-form>
      </q-card-section>
    </q-card>
  </q-page>

  <!--
  <ProgressDialog v-model="showProgress" />
--></template>

<script lang="ts">
import { defineComponent } from "vue";
// import ProgressDialog from "@/components/dialog/ProgressDialog.vue";
import * as git2rs from "@/api/git2rs";

export default defineComponent({
  /*
  components: {
    ProgressDialog,
  },
*/

  data() {
    return {
      form: {
        gitURL: "",
        directory: "",
      },
      showProgress: false,
    };
  },

  methods: {
    gitClone() {
      // const gitURL = "https://github.com/rust-lang/git2-rs.git";
      // const localDir = "/home/yjseo/tmp/test";
      const gitURL = this.form.gitURL;
      const localDir = this.form.directory;

      this.showProgress = true;
      git2rs
        .clone(gitURL, localDir)
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

    onSubmit() {
      this.gitClone();
    },
    onReset() {
      this.form.gitURL = "";
      this.form.directory = "";
    },
  },
});
</script>
