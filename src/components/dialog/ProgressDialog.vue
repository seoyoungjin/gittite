<template>
  <q-dialog ref="dialog" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <!-- progress -->
      <q-card-actions>
        <q-linear-progress size="25px" :value="progress" color="accent">
          <div class="absolute-full flex flex-center">
            <q-badge color="white" text-color="accent" :label="progressLabel" />
          </div>
        </q-linear-progress>
      </q-card-actions>

      <q-card-actions>
        <div>
          <h6>PROGRESS events</h6>
          <ol>
            <li v-for="item in inputs" :key="item">
              {{ item }}
            </li>
          </ol>
        </div>
      </q-card-actions>

      <!-- buttons example -->
      <q-card-actions align="right">
        <q-btn color="primary" label="OK" @click="onOKClick" />
        <q-btn color="primary" label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { listen } from "@tauri-apps/api/event";
import { ref, computed } from "vue";

export default {
  setup() {
    const inputs = ref([]);
    const progress = ref(0.2);

    const unlisten = listen("PROGRESS", (event) => {
      console.log("progress: " + JSON.stringify(event));
      let input = event.payload;
      inputs.value.push({ timestamp: Date.now(), payload: input });
      if (input.progress.progress == 0) {
        inputs.value = [];
        this.show();
      }
      progress.value = input.progress.progress / 100.0;
    });

    const onProgressReset = () => {
      inputs.value = [];
      progress.value = 0.0;
    };

    return {
      inputs,
      progress,
      progressLabel: computed(() => (progress.value * 100).toFixed(2) + "%"),

      onProgressReset,
    };
  },

  data() {
    return {};
  },

  methods: {
    show() {
      this.$refs.dialog.show();
    },
    hide() {
      this.$refs.dialog.hide();
    },
    onOKClick() {
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },
  },
};
</script>
