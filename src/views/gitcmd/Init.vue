<template>
  <q-page class="q-ma-lg">
    <h6>Init</h6>

    <q-card>
      <q-card-section>

        <form v-on:submit="gitInit" id="git-init">
          <div class="form-group">
            <label for="directory">Directory</label>
            <input class="form-control" v-model="form.directory" placeholder="Enter directory">
          </div>
          <div class="form-group form-check">
            <input type="checkbox" class="form-check-input" v-model="form.bareCheck">
            <label class="form-check-label" for="bareCheck">Bare</label>
          </div>
          <div class="form-group">
            <input type="checkbox" class="form-check-input" v-model="form.templateCheck">
            <label class="form-check-label" for="templateCheck">Template directory</label>
            <input class="form-control" v-model="form.templateDir" placeholder="Enter template directory">
          </div>
          <div class="form-group">
            <input type="checkbox" v-model="form.separateGitCheck">
            <label class="form-check-label" for="separateGitCheck">Separate git directory</label>
            <input class="form-control" v-model="form.separateGitDir" placeholder="Separate git directory">
          </div>
        </form>

      </q-card-section>
    </q-card>

    <br />

    <q-btn type=submit color="primary" no-caps @click="gitInit">Confirm</q-btn>
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
        directory: '',
        bareCheck: false,
        templateCheck:false,
        templateDir: '',
        seprateGitCheck: false,
        seprateGitDir: '',
      },
      response: null
    }
  },
  methods: {
    gitInit: function() {
      // alert(JSON.stringify(this.form, null, 4));
      // TODO argument
      var dirname = this.form.directory;
      invoke('init', {args: [dirname]}).then((message) => {
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
