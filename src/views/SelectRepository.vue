<template>
  <div class="q-pa-md">
    <q-layout view="lHh lpr lFf" container style="height: 80vh">
      <q-header bordered class="bg-white text-black">
        <q-toolbar>
          <q-toolbar-title> Select Repository </q-toolbar-title>
        </q-toolbar>
      </q-header>

      <q-page-container>
        <q-page class="q-pa-md">
          <div class="row q-gutter-md">
            <div class="col-5">
              <br />
              <div class="column q-gutter-lg" align="right">
                <q-btn
                  no-caps
                  color="primary"
                  align="left"
                  style="width: 250px"
                  @click="showCloneReposity = true"
                >
                  <OctIcon left size="2em" symbol="repoClone" />
                  <div>Clone a repository...</div>
                </q-btn>

                <q-btn
                  no-caps
                  color="primary"
                  align="left"
                  style="width: 250px"
                  @click="showInitReposity = true"
                >
                  <OctIcon left size="2em" symbol="plus" />
                  Create a new repository...
                </q-btn>

                <q-btn
                  no-caps
                  color="primary"
                  align="left"
                  style="width: 250px"
                  @click="showAddLocalReposity = true"
                >
                  <OctIcon left size="2em" symbol="fileDirectory" />
                  Add a local repository...
                </q-btn>
              </div>
            </div>

            <div class="col-auto">
              <div class="q-pa-md row justify-center">
                <q-list bordered>
                  <RepositoryItem
                    v-for="repo in allRepository"
                    :key="repo"
                    :repo="repo"
                  />
                </q-list>
              </div>
            </div>
          </div>
        </q-page>
      </q-page-container>
    </q-layout>
  </div>

  <InitRepository v-model="showInitReposity" />
  <AddLocalRepository v-model="showAddLocalReposity" />
  <CloneRepository v-model="showCloneReposity" />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mapActions, mapState } from "pinia";
import { useSettingsStore } from "@/stores/settings";
import { useAppStore } from "@/stores/app";
import RepositoryItem from "@/components/RepositoryItem.vue";
import InitRepository from "@/components/dialog/InitRepository.vue";
import AddLocalRepository from "@/components/dialog/AddLocalRepository.vue";
import CloneRepository from "@/components/dialog/CloneRepository.vue";
import OctIcon from "@/components/OctIcon.vue";

export default defineComponent({
  name: "SelectRepository",

  components: {
    OctIcon,
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
    ...mapActions(useAppStore, ["setPropModal"]),
  },
});
</script>
