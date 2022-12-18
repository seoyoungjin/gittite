<template>
  <div class="text-h6 q-mb-md">Profile</div>
  <q-form @submit="onSubmit" @reset="onReset" class="q-gutter-md">
    <q-input
      v-model="authorName"
      label="Name"
      placeholder="Enter your name"
      stack-label
      dense
    />
    <q-input
      v-model="authorEmail"
      label="Email"
      placeholder="Enter your email address"
      stack-label
      dense
    />
    <q-input
      v-model="authorImage"
      label="Image"
      placeholder="Paste your image"
      stack-label
      dense
    />
  </q-form>
</template>

<script lang="ts">
import { mapActions, mapState } from "pinia";
import { useSettingsStore } from "@/stores/settings";

export default {
  name: "Profile",

  data() {
    // TODO
    return {
      authorName: "",
      authorEmail: "",
      authorImage: "",
    };
  },

  computed: {
    ...mapState(useSettingsStore, ["getProfile"]),
  },

  methods: {
    ...mapActions(useSettingsStore, ["setProfile"]),
    ...mapActions(useSettingsStore, ["saveSettings"]),

    saveProfile() {
        // alert(JSON.stringify(this.getProfile));
        this.setProfile(this.authorName, this.authorEmail, this.authorImage);
        this.saveSettings();
    }
  },
};
</script>
