<template>
  <h6>Clone</h6>

  <q-card>
     <q-card-section>

       <form id="git-init">
         <div class="form-group">
           <label for="gitURL">Git URL</label>
           <input class="form-control" name="gitURL" placeholder="Enter directory" />
         </div>
         <div class="form-group">
           <label for="directory">Directory</label>
           <input class="form-control" name="directory" placeholder="Enter directory">
         </div>
         <div class="form-group form-check">
           <input type="checkbox" class="form-check-input" name="bareCheck">
           <label class="form-check-label" for="bareCheck">Bare</label>
         </div>
         <div class="form-group">
           <input type="checkbox" class="form-check-input" name="templateCheck">
           <label class="form-check-label" for="templateCheck">Template directory</label>
           <input class="form-control" name="templateDir" placeholder="Enter template directory">
         </div>
         <div class="form-group">
           <input type="checkbox" name="separateGitDir">
           <label class="form-check-label" for="separateGitDir">Separate git directory</label>
           <input class="form-control" name="templateDir" placeholder="Separate git directory">
         </div>
       </form>

    </q-card-section>
  </q-card>

  <br />

  <q-btn color="primary" no-caps @click="gitClone">Clone</q-btn>
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
      response: null
    }
  },

  methods: {
    gitClone() {
      const inputs = document.getElementById("git-clone").elements;
      const gitURL = inputs["gitURL"].value;
      const localDir = inputs["localDir"].value;

      invoke('clone', { args: [ "--url=" + gitURL, localDir ] })
      .then((message) => {
        response = message;
      })
      .catch(onMessage)
    }
  }
}
</script>
