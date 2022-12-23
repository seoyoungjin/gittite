<template>
  <q-dialog ref="dialog" @show="onDialogShow" @hide="onDialogHide">
    <q-card style="width: 600px">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Preference</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pt-none">
        <q-splitter v-model="splitterModel" style="height: 250px">
          <template v-slot:before>
            <q-tabs v-model="tab" inline-label vertical class="text-blue">
              <q-tab no-caps name="profile" icon="settings" label="Profile" />
            </q-tabs>
          </template>

          <template v-slot:after>
            <q-tab-panels v-model="tab" animated swipeable vertical>
              <q-tab-panel name="profile">
                <PrefProfile ref="profileRef" />
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
import DialogMixin from "@/mixins/dialog";
import PrefProfile from "@/components/preference/Profile.vue";

export default defineComponent({
  name: "Preference",
  mixins: [DialogMixin],

  setup() {
    return {
      tab: ref("profile"),
      splitterModel: ref(25),
    };
  },

  components: {
    PrefProfile,
  },

  data() {
    return {};
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
