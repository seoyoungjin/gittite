<template>
  <div class="text-h6 q-pa-none">Profile</div>
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
    return {
      authorName: "",
      authorEmail: "",
      authorImage: "",
    };
  },

  mounted() {
    this.authorName = this.getProfile.name;
    this.authorEmail = this.getProfile.email;
    this.authorImage = this.getProfile.image_url;
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
    },
  },
};
</script>
