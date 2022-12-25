<template>
  <q-toolbar class="bg-grey-10 text-grey-2">
    <q-btn flat no-caps no-wrap class="q-pa-xs">
      <!--
      <img alt="Vue logo" src="@/assets/logo.svg" width="25" height="25" />
      -->
      <q-toolbar-title shrink class="text-weight-bold">
        Gittite
      </q-toolbar-title>
    </q-btn>

    <q-space />

    <q-btn-dropdown flat no-caps class="q-pa-none">
      <template v-slot:label>
        <div class="row items-center no-wrap">
          <q-icon :name="octRepo16" size="16pt" class="q-pa-sm" />
          <q-item-section align="left">
            <q-item-label>
              <small>Current Repository</small>
            </q-item-label>
            <q-item-label>{{ repositoryPath }}</q-item-label>
          </q-item-section>
        </div>
      </template>
      <q-list dense>
        <q-item
          v-for="repo in allRepository"
          :key="repo"
          clickable
          v-close-popup
          @click="onItemClick(repo)"
        >
          <q-item-section>
            <q-item-label>{{ repo }}</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-btn-dropdown>
  </q-toolbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import { useSettingsStore } from "@/stores/settings";
import { octRepo16 } from "quasar-extras-svg-icons/oct-icons-v17";

export default defineComponent({
  name: "ToolBar",

  setup() {
    return {
      octRepo16,
    };
  },
  computed: {
    ...mapState(useRepositoryStore, ["repositoryPath"]),
    ...mapState(useSettingsStore, ["allRepository"]),
  },
  methods: {
    ...mapActions(useRepositoryStore, ["setRepository"]),

    onItemClick(repo: string) {
      this.setRepository(repo)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
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
