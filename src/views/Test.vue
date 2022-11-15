<script>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useQuasar } from "quasar";
import ProgressDialog from "@/components/dialog/ProgressDialog.vue";

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
      progressLabel1: computed(() => (progress1.value * 100).toFixed(2) + '%'),

      showProgress: false,
    };
  },

  methods: {
    testProgress() {
      invoke("test_progress")
        .then((message) => {
          this.response = message;
        })
        .catch((e) => {
          if (e) {
            this.response = { error: JSON.stringify(e) };
          }
        });
    },

    onReset() {
      this.inputs = [];
      this.progress1.value = 0.0;
    }
  },
};
</script>

<template>
  <h4>Test</h4>

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
</template>
