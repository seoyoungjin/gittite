<template>
  <q-page class="q-ma-lg">
    <h6>Stage Add/Remove</h6>

    <q-card>
      <q-card-section>
        <q-form @submit="onSubmit" @reset="onReset" class="q-gutter-md">
          <q-input v-model="file" label="File to add" hint="Enter file name" />
          <div>
            <q-btn label="Add" type="submit" color="primary"/>
            <q-btn label="Remove" type="reset" color="primary"/>
          </div>
        </q-form>
      </q-card-section>
    </q-card>

    <br />
    <q-btn color="primary" no-caps @click="getStatus('stage')"> Stage</q-btn>
    <q-btn color="primary" no-caps @click="getStatus('workdir')"> Work Dir</q-btn>
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
import * as git2rs from '../../api/git2rs.ts';

export default {
  components: {
    VueJsonPretty,
  },
  data() {
    return {
      response: null,
      file : null
    }
  },

  methods: {
    onSubmit () {
      this.gitAdd();
    },

    onReset () {
      this.file = null;
    },

    gitAdd() {
      git2rs.add().then((message) => {
        this.response = message;
      }).catch((e) => {
        if (typeof e == 'string') {
          this.response = {"error": e};
        } else {
          this.response = {"error": JSON.stringify(e)};
        }
      });
    },

    getStatus(args: string) {
      invoke('get_status', {statusType: args}).then((message) => {
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
