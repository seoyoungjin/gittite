<template>
  <q-page class="q-ma-lg">
    <h6>Stage Add/Remove</h6>

    <q-card>
      <q-card-section>
        <q-form @reset="onReset" class="q-gutter-md">
          <q-input v-model="file" label="File to add" hint="Enter file name" />
          <q-btn label="Add" @click='onGitAdd' color="primary"/>
          <q-btn label="Unstage" @click='onGitReset' color="primary"/>
          <q-btn label="Reset" type="reset" color="primary"/>
        </q-form>
      </q-card-section>
    </q-card>

    <br />

    <div>
      <vue-json-pretty :data=response />
    </div>
    <br />

    <div>
      <q-splitter v-model="splitterModel">

        <template v-slot:before>
          <div class="text-h5 q-mb-md">Unstaged</div>
          <div>
            <vue-json-pretty :data=unstagedJson />
          </div>
        </template>

        <template v-slot:after>
          <div class="text-h5 q-mb-md">Staged</div>
          <div>
            <vue-json-pretty :data=stagedJson />
          </div>
        </template>

      </q-splitter>
    </div>

  </q-page>
</template>

<script lang="ts">
import { ref } from 'vue'
import 'vue-json-pretty/lib/styles.css';
import VueJsonPretty from 'vue-json-pretty';
import { invoke } from '@tauri-apps/api/tauri';
import * as git2rs from '../../api/git2rs';

export default {
  components: {
    VueJsonPretty,
  },

  data() {
    return {
      response: null,
      stagedJson: null,
      unstagedJson: null,

      file : null,
      splitterModel: ref(50)
    }
  },

  mounted() {
    this.refreshStatus();
  },

  methods: {
    refreshStatus() {
      (async () => {
        this.stagedJson = await this.getStatus('stage');
        // alert(JSON.stringify(this.stagedJson, null, 4));
        this.unstagedJson = await this.getStatus('workdir');
      })();
    },

    onReset() {
      this.file = null;
      this.response = null;
      this.refreshStatus();
    },

    onGitAdd() {
      var name = this.file;
      git2rs.add(name).then((message) => {
        this.response = message;
      }).catch((e) => {
        if (typeof e == 'string') {
          this.response = {"error": e};
        } else {
          this.response = {"error": JSON.stringify(e)};
        }
      });
      this.refreshStatus();
    },

    onGitReset() {
      var name = this.file;
      git2rs.resetStage(name).then((message) => {
        this.response = message;
      }).catch((e) => {
        if (typeof e == 'string') {
          this.response = {"error": e};
        } else {
          this.response = {"error": JSON.stringify(e)};
        }
      });
      this.refreshStatus();
    },

    async getStatus(args: string) {
      try {
        return await invoke('get_status', {statusType: args});
      }
      catch (e) {
        if (typeof e == 'string') {
          return {"error": e};
        } else {
          return {"error": JSON.stringify(e)};
        }
      };
    }
  }
}
</script>
