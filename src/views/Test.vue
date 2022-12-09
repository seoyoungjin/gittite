<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { ref, computed } from "vue";
import { useQuasar } from "quasar";
import ProgressDialog from "@/components/dialog/ProgressDialog.vue";
import * as git2rs from "@/api/git2rs";

export default {
  components: {
    ProgressDialog,
  },

  setup() {
    const inputs = ref([]);
    const $q = useQuasar();
    const progress1 = ref(0.0);

    const unlisten = listen("PROGRESS", (event) => {
      console.log("progress: " + JSON.stringify(event));
      let input = event.payload;
      inputs.value.push({ timestamp: Date.now(), payload: input });
      if (input.progress.progress == 0) {
        inputs.value = [];
      }
      progress1.value = input.progress.progress / 100.0;
    });

    return {
      inputs,

      progress1,
      progressLabel1: computed(() => (progress1.value * 100).toFixed(2) + "%"),
      showProgress: false,

      propOptions: [
        'CWD', 'modal', 'Invalid'
      ]
    };
  },

  data() {
    return {
      propKey: "",
      propValue: "",
      propRes: null,

      progressRes: null,
    };
  },

  methods: {
    async getProp(key: string) {
      this.propRes = await git2rs.get_prop(key).catch((e) => {
        return e;
      });
    },

    async setProp(key: string, value: string) {
      this.propRes = key;
    },

    testProgress() {
      invoke("test_progress")
        .then((message) => {
          this.progressRes = message;
        })
        .catch((e) => {
          if (e) {
            this.progressRes = { error: JSON.stringify(e) };
          }
        });
    },

    onReset() {
      this.inputs = [];
      this.progress1.value = 0.0;
    },
  },
};
</script>

<template>
  <div class="q-ma-lg">
    <h4>Test</h4>

    <h6>Prop</h6>

    <div class="q-gutter-md row">
      <q-select
        v-model="propKey"
        :options="propOptions"
        label="Prop key"
        style="width: 40%"
      />
      <q-input
        v-model="propValue"
        label="Prop value"
        hint="Enter property value"
        style="width: 40%"
      />
    </div>
    <div>
      <q-btn label="Get" @click="getProp(propKey)" color="primary" />
      <q-btn label="Set" @click="setProp(propKey, propValue)" color="primary" />
    </div>
    <br />
    <div>
      {{ propRes }}
    </div>
    <br />

    <h6>Progress</h6>
    <div>
      <q-btn label="Progress" @click="testProgress" color="primary" />
      <q-btn label="Reset" @click="onReset" color="primary" />
    </div>
    <div class="q-pa-md">
      <q-linear-progress size="25px" :value="progress1" color="accent">
        <div class="absolute-full flex flex-center">
          <q-badge color="white" text-color="accent" :label="progressLabel1" />
        </div>
      </q-linear-progress>
    </div>
    <br />

    <br />
    <div>
      {{ inputs }}
    </div>
    <br />

    <br />

    <h6>Progress Dialog</h6>
    <div>
      <q-btn label="Progress" @click="showProgress = true" color="primary" />
    </div>
    <br />

    <ProgressDialog v-model="showProgress" />
  </div>
</template>
