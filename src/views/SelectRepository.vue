<template>
  <div class="q-pa-lg doc-container">
    <div class="text-h6 row justify-center">Select Repository</div>
    <div class="q-pa-md row justify-center">
      <q-list
        bordered
        style="min-width: 50vw; max-height: 80vh"
        class="rounded-borders"
      >
        <RepositoryItem
          v-for="repo in allRepository"
          :key="repo"
          :repo="repo"
        />
      </q-list>
    </div>
    <div class="row justify-center">
      <q-btn
        no-caps
        rounded
        color="primary"
        @click="showAddLocalReposity = true"
      >
        Add Repository
      </q-btn>
    </div>
  </div>

  <InitRepository v-model="showInitReposity" />
  <AddLocalRepository v-model="showAddLocalReposity" />
  <CloneRepository v-model="showCloneReposity" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import { useSettingsStore } from "@/stores/settings";
import { usePropStore } from "@/stores/props";
import RepositoryItem from "@/components/RepositoryItem.vue";
import InitRepository from "@/components/dialog/InitRepository.vue";
import AddLocalRepository from "@/components/dialog/AddLocalRepository.vue";
import CloneRepository from "@/components/dialog/CloneRepository.vue";

export default defineComponent({
  name: "SelectRepository",

  components: {
    RepositoryItem,
    // dialog
    InitRepository,
    AddLocalRepository,
    CloneRepository,
  },

  computed: {
    ...mapState(useSettingsStore, ["allRepository"]),
  },

  mounted() {
    this.setPropModal(true);
  },

  unmounted() {
    this.setPropModal(false);
  },

  data() {
    return {
      showInitReposity: false,
      showAddLocalReposity: false,
      showCloneReposity: false,
    };
  },

  methods: {
    ...mapActions(usePropStore, ["setPropModal"]),
  },
});
</script>
