<template>
  <q-page class="q-ma-lg">
    <h6>Init</h6>

    <q-card>
      <q-card-section>
        <q-form v-on:submit="gitInit" id="git-init">
          <q-input
            v-model="form.directory"
            label="Directory"
            hint="Enter directory"
          />
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
        </q-form>
      </q-card-section>
    </q-card>

    <br />

    <q-btn type="submit" color="primary" no-caps @click="gitInit"
      >Confirm</q-btn
    >
    <br />
    <br />

    <div v-if="response">
      <vue-json-pretty :data="response" />
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import * as git2rs from "../../lib/git2rs";

export default defineComponent({
  components: {
    VueJsonPretty,
  },
  data() {
    return {
      form: {
        directory: "",
        bareCheck: false,
        templateCheck: false,
        templateDir: "",
        separateGitCheck: false,
        separateGitDir: "",
      },
      response: null,
    };
  },
  methods: {
    gitInit: function () {
      // alert(JSON.stringify(this.form, null, 4));
      // TODO argument
      var dirname = this.form.directory;
      git2rs
        .init(dirname)
        .then((message) => {
          this.response = message as any;
        })
        .catch((e) => {
          if (e) {
            this.response = { error: JSON.stringify(e) } as any;
          }
        });
    },
  },
});
</script>
