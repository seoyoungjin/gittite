<template>
  <q-dialog ref="dialog" @show="setModal" @hide="unsetModal">
    <q-card class="q-dialog-plugin" style="min-width: 600px">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Repository Settings</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pa-none">
        <q-splitter v-model="splitterModel" style="min-height: 250px">
          <template v-slot:before>
            <q-list>
              <q-item
                v-for="item in sections"
                clickable
                v-ripple
                :key="item.link"
                :active="tab === item.link"
                @click="tab = item.link"
                active-class="text-white bg-primary"
              >
                <q-item-section>{{ item.text }}</q-item-section>
              </q-item>
            </q-list>
          </template>

          <template v-slot:after>
            <q-tab-panels v-model="tab" animated swipeable vertical>
              <q-tab-panel name="remote" class="q-gutter-md">
                <div>Primary remote repository (origin)</div>
                <q-input v-model="remoteUrl" dense outlined />
              </q-tab-panel>

              <q-tab-panel name="gitignore" class="q-gutter-md">
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
                  :options="config_options"
                  type="radio"
                  v-model="config_group"
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
      splitterModel: ref(30),

      tab: ref("remote"),
      sections: [
        { text: "Remote", link: "remote" },
        { text: "Ignored Files", link: "gitignore" },
        { text: "Git Config", link: "gitconfig" },
      ],

      config_group: ref("global"),
      config_options: [
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
