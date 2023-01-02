<template>
  <q-dialog ref="dialog" @show="setModal" @hide="unsetModal">
    <q-card class="q-dialog-plugin">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Delete Branch</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pa-md">
        <q-card-actions class="q-gutter-sm" vertical>
          <div class="text-subtitle2">Local Branches</div>
          <q-list dense bordered>
            <q-item
              v-for="branch in allBranches"
              :key="branch.name"
              clickable
              :active="branch.name == branchSelected"
              active-class="text-white bg-blue"
              @click="onBranchClick(branch.name)"
            >
              <q-item-section>{{ branch.name }}</q-item-section>
            </q-item>
          </q-list>
        </q-card-actions>
      </q-card-section>

      <q-separator />

      <q-card-actions align="right">
        <q-btn
          no-caps
          color="primary"
          label="OK"
          @click="onOKClick"
          :disable="!branchSelected"
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
import * as git2rs from "@/api/git2rs";
import { useRepositoryStore } from "@/stores/repository";

export default defineComponent({
  name: "BranchDelete",
  mixins: [ModalMixin],

  data() {
    return {
      branchSelected: "",
    };
  },

  emits: ["ok"],

  computed: {
    ...mapState(useRepositoryStore, ["allBranches"]),
  },

  methods: {
    ...mapActions(useRepositoryStore, ["getBranchInfo", "loadAllBranches"]),

    show() {
      (this.$refs.dialog as any).show();
    },
    hide() {
      (this.$refs.dialog as any).hide();
    },
    onOKClick() {
      this.deleteBranch(this.branchSelected);
      this.$emit("ok");
      this.hide();
    },
    onCancelClick() {
      this.hide();
    },

    // dialog specific
    onBranchClick(branchName: string) {
      this.branchSelected = branchName;
    },
    deleteBranch(branchName: string) {
      var info = this.getBranchInfo(branchName);
      git2rs
        .deleteBranch(info.reference)
        .then(() => {
          var message = branchName + " branch deleted";
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
