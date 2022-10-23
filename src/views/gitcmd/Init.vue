<template>
  <h6>Init</h6>

  <q-card>
     <q-card-section>

       <form v-on:submit="gitInit" id="git-init">
         <div class="form-group">
           <label for="directory">Directory</label>
           <input class="form-control" v-model="directory" placeholder="Enter directory">
         </div>
         <div class="form-group form-check">
           <input type="checkbox" class="form-check-input" v-model="bareCheck">
           <label class="form-check-label" for="bareCheck">Bare</label>
         </div>
         <div class="form-group">
           <input type="checkbox" class="form-check-input" v-model="templateCheck">
           <label class="form-check-label" for="templateCheck">Template directory</label>
           <input class="form-control" v-model="templateDir" placeholder="Enter template directory">
         </div>
         <div class="form-group">
           <input type="checkbox" v-model="separateGitCheck">
           <label class="form-check-label" for="separateGitCheck">Separate git directory</label>
           <input class="form-control" v-model="separateGitDir" placeholder="Separate git directory">
         </div>

         <button type="submit">login</button>
       </form>

    </q-card-section>
  </q-card>

  <br />

  <q-btn type=submit color="primary" no-caps @click="gitInit">Confirm</q-btn>
  <br />

  <div>
    <vue-json-pretty :data=response />
  </div>
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
      directory: '',
      bareCheck: false,
      templateCheck:false,
      templateDir: '',
      seprateGitCheck: false,
      seprateGitDir: '',
      response: null
    }
  },
  methods: {
    gitInit: function() {
      var dirname = this.directory;
      // alert(dirname);
      // TODO argument
      invoke('init', {args: ["dummy", dirname]}).then((message) => {
        response = message;
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
