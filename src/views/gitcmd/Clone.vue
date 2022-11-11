<template>
  <q-page class="q-ma-lg">
    <h6>Clone</h6>

    <q-card>
      <q-card-section>
        <q-form @submit="onSubmit" @reset="onReset" class="q-gutter-md">
          <q-input v-model="form.gitURL" label="gitURL" hint="Enter git url" />
          <q-input
            v-model="form.directory"
            label="Directory"
            hint="Enter directory"
          />

          <div>
            <q-btn label="Submit" type="submit" color="primary" />
            <q-btn label="Reset" type="reset" color="primary" flat class="q-ml-sm" />
          </div>
        </q-form>
      </q-card-section>
    </q-card>

    <div>
      <h6>PROGRESS events</h6>
      <ol>
        <li v-for="input in inputs">
          {{input}}
        </li>
      </ol>
    </div>
  </q-page>

</template>

<script lang="ts">
import { ref } from 'vue'
import { useQuasar } from "quasar";
import { listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window';
import ProgressDialog from "../../components/ProgressDialog"
import * as git2rs from '../../api/git2rs'

export default {
  setup() {
    const inputs = ref([]);
    const $q = useQuasar()

    listen('PROGRESS', (event) => {
      console.log("js: rs2js: " + event);
      let input = event.payload;
      inputs.value.push({ timestamp: Date.now(), payload: input })
      if (input.message == 'start') {
        showProgress();
      }
    });

    function showProgress () {
      const dialog = $q.dialog({
        message: 'Uploading... 0%',
        progress: true, // we enable default settings
        persistent: true, // we want the user to not be able to close it
        ok: false // we want the user to not be able to close it
      })

      // we simulate some progress here...
      let percentage = 0
      const interval = setInterval(() => {
        percentage = Math.min(100, percentage + Math.floor(Math.random() * 22))

        // we update the dialog
        dialog.update({
          message: `Uploading... ${percentage}%`
        })

        // if we are done, we're gonna close it
        if (percentage === 100) {
          clearInterval(interval)
          setTimeout(() => {
            dialog.hide()
          }, 350)
        }
      }, 500)
    }

    return {
      inputs,
    }
  },

  data() {
    return {
      form: {
        gitURL: null,
        directory: null,
      },
    };
  },

  methods: {
    gitClone() {
      const gitURL = this.form.gitURL;
      const localDir = this.form.directory;

      git2rs.clone(gitURL, localDir)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
        })
        .catch((e) => {
          var message =JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });
    },

    onSubmit() {
      this.gitClone();
    },
    onReset() {
      this.form.gitURL = null;
      this.form.directory = null;
    },
  },
};
</script>
