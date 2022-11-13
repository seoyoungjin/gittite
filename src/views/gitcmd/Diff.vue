<template>
  <q-page class="q-ma-lg">
    <h6>Diff</h6>

    <q-card>
      <q-card-section>
        <q-form v-on:submit="getDiff" id="git-diff">
          <q-input v-model="form.filename" label="File" hint="Enter file name" />
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

    <q-btn type=submit color="primary" no-caps @click="gitDiff">Confirm</q-btn>
    <br />
    <br />

    <div>
      <vue-json-pretty :data=response />
    </div>
  </q-page>
</template>

<script lang="ts">
import 'vue-json-pretty/lib/styles.css';
import VueJsonPretty from 'vue-json-pretty';
import { invoke } from '@tauri-apps/api/tauri';

export default {
  components: {
    VueJsonPretty,
  },
  data() {
    return {
      form : {
        file: '',
        stageCheck: false,
      },
      response: null
    }
  },
  methods: {
    gitDiff: function() {
      var filename = this.form.filename;
      invoke('get_diff', {args: filename}).then((message) => {
        this.response = message;
      }).catch((e) => {
        if (typeof e == 'string') {
          this.response = {"error": e};
        } else {
          this.response = {"error": JSON.stringify(e)};
        }
      });
    }
  }
}
</script>
