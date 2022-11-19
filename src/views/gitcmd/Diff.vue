<template>
  <q-page class="q-ma-lg">
    <h6>Diff</h6>

    <q-card>
      <q-card-section>
        <q-form v-on:submit="getDiff" id="git-diff">
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
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  components: {
    VueJsonPretty,
  },
  data() {
    return {
      form: {
        file: "",
        stageCheck: false,
      },
      response: null,
      error: null,
    };
  },
  methods: {
    gitDiff: function () {
      var filename = this.form.filename;
      // invoke("get_diff", { args: filename })
      invoke("diff", { args: [] })
        .then((message) => {
          this.response = message;
        })
        .catch((e) => {
          if (e) {
            this.error = { error: JSON.stringify(e) };
          }
        });
    },
  },
};
</script>
