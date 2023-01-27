<script lang="ts">
import { defineComponent, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import RemoteProgress from "@/components/RemoteProgress.vue";
import ProgressDialog from "@/components/dialog/ProgressDialog.vue";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  components: {
    RemoteProgress,
    ProgressDialog,
  },

  setup() {
    const inputs = ref([]);
    const progress = ref(0.0);
    let unlisten = null as any;

    const startListen = async () => {
      if (unlisten) return;
      unlisten = await listen("PROGRESS", (event) => {
        console.log("progress: " + JSON.stringify(event));
        let payload = event.payload as any;
        inputs.value.push(payload as never);
        progress.value = payload.progress.progress / 100.0;
      });
    };

    const stopListen = () => {
      if (unlisten) {
        unlisten();
        unlisten = null as any;
      }
    };

    const resetProgress = () => {
      inputs.value = [];
      progress.value = 0.0;
      stopListen();
    };

    return {
      unlisten,
      progress,
      inputs,

      startListen,
      stopListen,
      resetProgress,
    };
  },

  computed: {
    progressLabel() {
      return (this.progress * 100).toFixed(2) + "%";
    },
  },

  data() {
    return {
      // get/set prop
      propOptions: ["CWD", "LastFetchedTime", "modal", "Invalid"],
      propKey: "",
      propValue: "",
      propRes: "",
      propError: null,
    };
  },

  methods: {
    async getProp(key: string) {
      this.propRes = await git2rs.get_prop(key).catch((e) => {
        this.propError = { error: JSON.stringify(e) } as any;
        return "";
      });
    },

    async setProp(key: string, value: string) {
      await git2rs.set_prop(key, value).catch((e) => {
        this.propError = { error: JSON.stringify(e) } as any;
      });
    },

    startProgress() {
      this.startListen();
      this.testProgress();
    },

    testProgress() {
      git2rs.testProgress();
    },

    // remote progress
    startRemoteProgress() {
      (this.$refs.progressRef as any).startProgress();
      git2rs.testProgress();
    },

    // progress dialog
    showProgressDialog() {
      (this.$refs.progressDialog as any).show();
      // (this.$refs.progressDialog as any).startRemoteProgress();
      this.testProgress();
    },
  },
});
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
    <div v-if="propRes">
      {{ propRes }}
    </div>
    <div v-if="propError">
      {{ propError }}
    </div>
    <br />

    <h6>Progress</h6>
    <div>
      <q-btn label="Start Listen" @click="startListen" color="primary" />
      <q-btn label="Stop Listen" @click="stopListen" color="primary" />
      <q-btn label="Test Progress" @click="testProgress" color="primary" />
      <q-btn label="Reset" @click="resetProgress" color="primary" />
    </div>
    <div class="q-pa-md">
      <q-linear-progress size="25px" :value="progress" color="accent">
        <div class="absolute-full flex flex-center">
          <q-badge color="white" text-color="accent" :label="progressLabel" />
        </div>
      </q-linear-progress>
    </div>
    <br />
    <div>
      <div v-for="item in inputs" :key="item">
        {{ item }}
      </div>
    </div>
    <br />

    <h6>Remote Progress Component</h6>
    <div>
      <q-btn label="Start" @click="startRemoteProgress" color="primary" />
      <q-btn
        label="Stop"
        @click="($refs.progressRef as any).stopProgress()"
        color="primary"
      />
      <q-btn
        label="Reset"
        @click="($refs.progressRef as any).resetProgress()"
        color="primary"
      />
    </div>
    <div class="q-pa-md">
      <RemoteProgress ref="progressRef" />
    </div>
    <br />

    <h6>Progress Dialog</h6>
    <div>
      <q-btn color="primary" label="Progress" @click="showProgressDialog" />
    </div>
    <br />

    <ProgressDialog ref="progressDialog" />
  </div>
</template>
