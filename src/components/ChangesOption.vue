<template>
  <div class="q-ma-none">

    Untracked file

    <q-btn-dropdown
      v-model="menu"
      class="glossy q-ml-sm"
      label="Un"
    >
      <q-list>
        <q-item clickable v-close-popup @click="onItemClick">
          <q-item-section>
            <q-item-label>Photos</q-item-label>
          </q-item-section>
        </q-item>
        <q-item clickable v-close-popup @click="onItemClick">
          <q-item-section>
            <q-item-label>Photos</q-item-label>
          </q-item-section>
        </q-item>
        <q-item clickable v-close-popup @click="onItemClick">
          <q-item-section>
            <q-item-label>Photos</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-btn-dropdown>

    Ingored

    <q-btn-dropdown
      v-model="menu"
      class="glossy q-ml-sm"
      label="Un"
    >
      <q-list>
        <q-item clickable v-close-popup @click="onItemClick">
          <q-item-section>
            <q-item-label>Photos</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-btn-dropdown>

    Submodule
    <q-btn-dropdown
      v-model="menu"
      class="glossy q-ml-sm"
      label="Un"
    >
      <q-list>
        <q-item clickable v-close-popup @click="onItemClick">
          <q-item-section>
            <q-item-label>Photos</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-btn-dropdown>

    <!--
    <div class="bg-grey-2 q-pa-sm rounded-borders">
      Preferred genre:
      <q-option-group
        name="untracked file"
        v-model="group1"
        :options="untracked"
        color="primary"
        inline
      />
    </div>
    <q-toggle v-model="value" label="Untracked file" />
    <q-toggle v-model="value" color="yellow" label="Ingored" />
    <q-toggle v-model="value" color="green" label="Ignore submodules" />
    -->
  </div>
</template>

<script lang="ts">
import { ref } from "vue";

export default {
  setup() {
    return {
      group1: ref(null),
      group2: ref(null),
      group3: ref(null),

      untracked: [
        { label: "No", value: "no" },
        { label: "Normal", value: "normal", color: "green" },
        { label: "All", value: "all", color: "red" },
      ],
      ingrored: [
        { label: "None", value: "none" },
        { label: "Untracked", value: "untracked", color: "green" },
        { label: "Dirty", value: "dirty", color: "red" },
        { label: "All", value: "all", color: "blue" },
      ],
      submodule: [
        { label: "Traditional", value: "traditional" },
        { label: "No", value: "no", color: "green" },
        { label: "Matching", value: "matching", color: "red" },
      ],
    };
  },

  methods: {},
};
</script>
