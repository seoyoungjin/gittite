<template>
  <q-item class="item-row">
    <q-item-section>
      <q-item-label>{{ repo }}</q-item-label>
    </q-item-section>

    <q-btn rounded no-caps color="primary" @click="onItemOpen(repo)">
      Open
    </q-btn>
    <q-btn rounded icon="delete" @click="onItemDelete(repo)" />
  </q-item>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions } from "pinia";
import { useSettingsStore } from "@/stores/settings";
import { useRepositoryStore } from "@/stores/repository";

export default defineComponent({
  name: "RepositoryItem",
  props: ["repo"],

  methods: {
    ...mapActions(useRepositoryStore, ["setRepository"]),
    ...mapActions(useSettingsStore, ["removeRepositoryFromSettings"]),

    onItemOpen(repo: string) {
      this.setRepository(repo)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
          this.$router.push("/");
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

    onItemDelete(repo: string) {
      this.removeRepositoryFromSettings(repo);
    },
  },
});
</script>
