<template>
  <q-page class="q-ma-lg">
    <h6>Diff</h6>

    <q-card>
      <q-card-section>
        <q-form v-on:submit="gitDiff" id="git-diff">
          <q-input
            v-model="form.filename"
            label="File"
            hint="Enter file name"
          />
          <q-checkbox v-model="form.stageCheck" label="Stage" />
        </q-form>
      </q-card-section>
    </q-card>

    <br />
    NOTE: This json encoding of diff format is too heavy.
    <pre>
    TODO
    - staged or workdir  check
    - diff in commit
    </pre>
    <br />
    <br />

    <q-btn type="submit" color="primary" no-caps @click="gitDiff"
      >Confirm</q-btn
    >
    <br />
    <br />

    <div v-if="response">
      <pre>
        {{ response }}
      </pre>
    </div>
    <div v-if="error">
      <vue-json-pretty :data="error" />
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
        filename: "",
        stageCheck: false,
      },
      response: null,
      error: null,
    };
  },
  methods: {
    gitDiff: function () {
      var filename = this.form.filename;
      var stageCheck = this.form.stageCheck;
      git2rs
        .getDiff(filename, stageCheck)
        .then((message) => {
          this.response = message as any;
        })
        .catch((e) => {
          if (e) {
            this.error = { error: JSON.stringify(e) } as any;
          }
        });
    },
  },
});
</script>
