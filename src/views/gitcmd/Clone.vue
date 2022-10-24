<template>
  <q-page class="q-ma-lg">
    <h6>Clone</h6>

    <q-card>
      <q-card-section>

        <q-form @submit="onSubmit" @reset="onReset" class="q-gutter-md">
          <q-input v-model="form.gitURL" label="gitURL" hint="Enter git url" />
          <q-input v-model="form.directory" label="Directory" hint="Enter directory" />

          <div>
            <q-btn label="Submit" type="submit" color="primary"/>
            <q-btn label="Reset" type="reset" color="primary" flat class="q-ml-sm" />
          </div>
        </q-form>

      </q-card-section>
    </q-card>

  </q-page>
</template>

<script lang="ts">
import { useQuasar } from "quasar";
import { invoke } from '@tauri-apps/api/tauri';

export default {
  data() {
    return {
      form : {
        gitURL: null,
        directory: null,
      }
    }
  },

  methods: {
    gitClone () {
      // alert(JSON.stringify(this.form, null, 4));
      const gitURL = this.form.gitURL;
      const localDir = this.form.directory;

      invoke('clone', {
        args: [ gitURL, localDir ] 
      }).then((message) => {
        this.$q.notify({
          color: 'green-5',
          textColor: 'white',
          icon: 'cloud',
          message: message
        })
      }).catch((e) => {
        var message = (typeof e == 'string')? e : JSON.stringify(e, null, 4)
        this.$q.notify({
          color: 'red-5',
          textColor: 'white',
          icon: 'warning',
          message: message
        })
      })
    },
    onSubmit () {
      this.gitClone();
    },
    onReset () {
      this.form.gitURL = null
      this.form.directory = null
    }
  }
}
</script>
