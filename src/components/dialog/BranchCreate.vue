<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="unsetModal">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Create a Branch</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section>
        <q-card-actions class="q-gutter-sm" vertical>
          <div class="text-subtitle2">Name</div>
          <q-input outlined v-model="branchName" dense />
        </q-card-actions>

        <q-card-actions vertical>
          <div>Create branch based on...</div>
          <q-list dense bordered separator padding>
            <q-item v-for="branch in branchItems" :key="branch.key">
              <q-item-section>
                <q-item-label class="text-weight-bold">
                  {{ branch.title }}</q-item-label
                >
                <q-item-label caption>{{ branch.description }}</q-item-label>
              </q-item-section>
            </q-item>
          </q-list>
        </q-card-actions>
      </q-card-section>

      <!-- error message -->
      <DialogError :message="currentError.message" v-if="currentError" />

      <q-separator />

      <q-card-actions align="right">
        <q-btn
          no-caps
          color="primary"
          label="OK"
          @click="onOKClick"
          :disable="disabled"
        />
        <q-btn no-caps label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import ModalMixin from "@/mixins/modal";
import DialogError from "@/components/DialogError.vue";
import * as git2rs from "@/lib/git2rs";
import { useRepositoryStore } from "@/stores/repository";

export default defineComponent({
  name: "BranchCreate",
  mixins: [ModalMixin],

  components: {
    DialogError,
  },

  data() {
    return {
      branchName: "",
      branchItems: [] as any[],
      defaultBranch: "",
      currentError: null as Error | null,
    };
  },

  emits: ["ok"],

  computed: {
    ...mapState(useRepositoryStore, ["currentBranch", "allBranches"]),

    disabled(): boolean {
      const disabled = this.branchName.length === 0 || !!this.currentError;
      return disabled;
    },
  },

  watch: {
    branchName: async function (val: string) {
      this.currentError = null;
      if (val.length === 0) return;
      const exists = this.allBranches.findIndex((b) => b.name === val) > -1;
      if (exists) {
        this.currentError = new Error(`A branch named ${val} already exists`);
        return;
      }
      const valid = await git2rs.validateBranchName(val);
      if (!valid) {
        this.currentError = new Error("Invalid branch name");
        return;
      }
    },
  },

  methods: {
    ...mapActions(useRepositoryStore, ["loadAllBranches"]),

    show() {
      (this.$refs.dialog as any).show();
    },
    hide() {
      (this.$refs.dialog as any).hide();
    },
    onOKClick() {
      this.createBranch(this.branchName);
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },

    // dialog specific
    onDialogShow() {
      this.setModal();
      this.branchItems = [
        {
          title: this.currentBranch,
          description: "The currently checked out branch.",
          key: this.currentBranch,
        },
      ];
      this.defaultBranch = this.currentBranch;
    },

    // onBranchClick(branchName: string) {
    //   this.defaultBranch = branchName;
    // },

    createBranch(name: string) {
      git2rs
        .createBranch(name)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
          // refresh
          this.loadAllBranches();
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });
    },
  },
});
</script>
