<template>
  <q-dialog ref="dialog" @hide="onDialogHide">
    <q-card class="q-dialog-plugin">
      <q-card-section>
        <div class="text-h6">Create a New Repository</div>
      </q-card-section>

      <q-separator />

      <q-card-actions vertical>
        <q-form id="git-init">
          <q-input
            v-model="form.name"
            label="Name"
            placeholder="repository name"
          />
          <q-input
            v-model="form.directory"
            label="Local directory"
            placeholder="Local directory"
          />
          <q-checkbox v-model="form.bareCheck" label="Bare" />
          <q-input
            v-model="form.templateDir"
            label="Template directory"
            :disable="!form.templateCheck"
          >
            <template v-slot:before>
              <q-checkbox v-model="form.templateCheck" />
            </template>
          </q-input>
          <q-input
            v-model="form.separateGitDir"
            label="Separate Git directory"
            :disable="!form.separateGitCheck"
          >
            <template v-slot:before>
              <q-checkbox v-model="form.separateGitCheck" />
            </template>
          </q-input>
        </q-form> 
      </q-card-actions>

      <q-card-actions align="right">
        <q-btn color="primary" label="OK" @click="onOKClick" />
        <q-btn color="primary" label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
export default {
  data() {
    return {
      form: {
        name: "",
        directory: "",
        bareCheck: false,
        templateCheck: false,
        templateDir: "",
        separateGitCheck: false,
        separateGitDir: "",
      },
    };
  },

  emits: [
    // REQUIRED
    'ok', 'hide'
  ],

  methods: {
    // following method is REQUIRED
    show () {
      this.$refs.dialog.show()
    },

    // following method is REQUIRED
    hide () {
      this.$refs.dialog.hide()
    },

    onDialogHide () {
      // required to be emitted
      // when QDialog emits "hide" event
      this.$emit('hide')
    },

    onOKClick () {
      // on OK, it is REQUIRED to
      // emit "ok" event (with optional payload)
      // before hiding the QDialog
      this.$emit('ok')
      // or with payload: this.$emit('ok', { ... })

      // then hiding dialog
      this.hide()
    },

    onCancelClick () {
      // we just need to hide the dialog
      this.hide()
    }
  }
}
</script>
