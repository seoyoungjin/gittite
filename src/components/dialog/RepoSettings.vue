<template>
  <q-dialog ref="dialog" @show="setModal" @hide="unsetModal">
    <q-card style="width: 600px">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Repository Settings</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pa-none">
        <q-splitter v-model="splitterModel" style="min-height: 250px">
          <template v-slot:before>
            <q-tabs no-caps v-model="tab" vertical class="text-teal">
              <q-tab name="remote" label="Remote" />
              <q-tab name="ignored" label="Ignored Files" />
              <q-tab name="gitconfig" label="Git Config" />
            </q-tabs>
          </template>

          <template v-slot:after>
            <q-tab-panels v-model="tab" animated swipeable vertical>
              <q-tab-panel name="remote" class="q-gutter-md">
                <div>Primary remote repository (origin)</div>
                <q-input v-model="remoteUrl" dense outlined />
              </q-tab-panel>

              <q-tab-panel name="ignored" class="q-gutter-md">
                <div>Editing <code>.gitigore</code>.</div>
                <q-input
                  v-model="ignoredText"
                  placeholder="Ignored files"
                  type="textarea"
                  outlined
                />
              </q-tab-panel>

              <q-tab-panel name="gitconfig" class="q-gutter-sm">
                <div class="text-bold">Repository Git config</div>
                <q-option-group
                  class="q-pa-none"
                  :options="options"
                  type="radio"
                  v-model="group"
                />
                <div>Name</div>
                <q-input v-model="accountName" dense outlined />
                <div>Email</div>
                <q-input v-model="accountEmail" dense outlined />
              </q-tab-panel>
            </q-tab-panels>
          </template>
        </q-splitter>
      </q-card-section>

      <q-separator />

      <q-card-actions align="right">
        <q-btn no-caps color="primary" label="OK" @click="onOKClick" />
        <q-btn no-caps label="Cancel" @click="onCancelClick" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import ModalMixin from "@/mixins/modal";

export default defineComponent({
  name: "RepoSettings",
  mixins: [ModalMixin],

  setup() {
    return {
      tab: ref("remote"),
      splitterModel: ref(30),

      group: ref("global"),
      options: [
        { label: "Use global Git config", value: "global" },
        { label: "Use a local Git config", value: "local" },
      ],
    };
  },

  data() {
    return {
      remoteUrl: "",
      ignoredText: "",
      accountName: "",
      accountEmail: "",
    };
  },

  emits: ["ok"],

  methods: {
    show() {
      (this.$refs.dialog as any).show();
    },

    hide() {
      (this.$refs.dialog as any).hide();
    },

    onOKClick() {
      (this.$refs.profileRef as any).saveProfile();
      this.$emit("ok");
      this.hide();
    },

    onCancelClick() {
      this.hide();
    },
  },
});
</script>
