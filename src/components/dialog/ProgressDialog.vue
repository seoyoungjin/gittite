<template>
  <q-dialog ref="dialogRef" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <!--
        ...content
        ... use q-card-section for it?
      -->

      <!-- buttons example -->
      <q-card-actions align="right">
        <q-btn color="primary" label="OK" @click="onOKClick" />
        <q-btn color="primary" label="Cancel" @click="onDialogCancel" />
      </q-card-actions>
    </q-card>

    <div>
      <h6>PROGRESS events</h6>
      <ol>
        <li v-for="input in inputs">
          {{ input }}
        </li>
      </ol>
    </div>
  </q-dialog>
</template>

<script setup>
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { useQuasar } from "quasar";
import { useDialogPluginComponent } from "quasar";

const inputs = ref([]);
const $q = useQuasar();

listen("PROGRESS", (event) => {
  console.log("js: rs2js: " + event);
  let input = event.payload;
  inputs.value.push({ timestamp: Date.now(), payload: input });
  if (input.message == "start") {
    showProgress();
  }
});

function showProgress() {
  const dialog = $q.dialog({
    message: "Uploading... 0%",
    progress: true, // we enable default settings
    persistent: true, // we want the user to not be able to close it
    ok: false, // we want the user to not be able to close it
  });

  // we simulate some progress here...
  let percentage = 0;
  const interval = setInterval(() => {
    percentage = Math.min(100, percentage + Math.floor(Math.random() * 22));

    // we update the dialog
    dialog.update({
      message: `Uploading... ${percentage}%`,
    });

    // if we are done, we're gonna close it
    if (percentage === 100) {
      clearInterval(interval);
      setTimeout(() => {
        dialog.hide();
      }, 350);
    }
  }, 500);
}

const props = defineProps({
  // ...your custom props
});

defineEmits([
  // REQUIRED; need to specify some events that your
  // component will emit through useDialogPluginComponent()
  ...useDialogPluginComponent.emits,
]);

const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } =
  useDialogPluginComponent();
// dialogRef      - Vue ref to be applied to QDialog
// onDialogHide   - Function to be used as handler for @hide on QDialog
// onDialogOK     - Function to call to settle dialog with "ok" outcome
//                    example: onDialogOK() - no payload
//                    example: onDialogOK({ /*...*/ }) - with payload
// onDialogCancel - Function to call to settle dialog with "cancel" outcome

// this is part of our example (so not required)
function onOKClick() {
  // on OK, it is REQUIRED to
  // call onDialogOK (with optional payload)
  onDialogOK();
  // or with payload: onDialogOK({ ... })
  // ...and it will also hide the dialog automatically
}
</script>
