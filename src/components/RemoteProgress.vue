<script lang="ts">
import { defineComponent } from "vue";
import { computed, ref } from "vue";
import { listen } from "@tauri-apps/api/event";

export default defineComponent({
  props: {
    message: {
      type: String,
      default: "",
    },
  },

  watch: {
    message: {
      handler(value: string) {
        if (value === "start") {
          this.startProgress();
        } else if (value === "stop") {
          this.stopProgress();
        } else if (value === "reset") {
          this.resetProgress();
        }
      },
      immediate: true,
    },
  },

  emits: ["progressDone"],

  setup(_, { emit }) {
    const progress = ref(0.0);
    const progressLabel = computed(() => {
      return (progress.value * 100).toFixed(2) + "%";
    });
    let unlisten = null as any;

    const startProgress = async () => {
      if (unlisten) return;
      unlisten = await listen("PROGRESS", (event) => {
        // console.log("progress: " + JSON.stringify(event));
        let payload = event.payload as any;
        progress.value = payload.progress.progress / 100.0;
        if (payload.progress.progress === 100) {
          emit("progressDone");
        }
      });
    };

    const stopProgress = () => {
      if (unlisten) {
        unlisten();
        unlisten = null as any;
      }
    };

    const resetProgress = () => {
      progress.value = 0.0;
      stopProgress();
    };

    return {
      unlisten,
      progress,
      progressLabel,

      // functions
      startProgress,
      stopProgress,
      resetProgress,
    };
  },
});
</script>

<template>
  <div class="q-pa-none">
    <q-linear-progress size="25px" :value="progress" color="accent">
      <div class="absolute-full flex flex-center">
        <q-badge color="white" text-color="accent" :label="progressLabel" />
      </div>
    </q-linear-progress>
  </div>
</template>
