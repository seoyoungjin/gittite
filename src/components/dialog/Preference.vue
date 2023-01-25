<template>
  <q-dialog ref="dialog" @show="setModal" @hide="unsetModal">
    <q-card style="width: 600px">
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Preference</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pa-none">
        <q-splitter v-model="splitterModel" style="height: 250px">
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
                <q-item-section avatar>
                  <q-icon :name="item.icon" />
                </q-item-section>
                <q-item-section>{{ item.text }}</q-item-section>
              </q-item>
            </q-list>
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
import ModalMixin from "@/mixins/modal";
import PrefProfile from "@/components/preference/Profile.vue";

export default defineComponent({
  name: "Preference",
  mixins: [ModalMixin],

  setup() {
    return {
      splitterModel: ref(30),
      tab: ref("profile"),
      sections: [
        { icon: "home", text: "Profile", link: "profile" },
        { icon: "home", text: "Test", link: "test" },
      ],
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
