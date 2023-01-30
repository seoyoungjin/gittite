<template>
  <div>
    <q-card>
      <q-card-section>
        <div class="text-weight-bold">Status Option</div>
        <q-select
          v-model="untracked"
          :options="untracked_opts"
          label="Untracked file"
        />
        <q-select v-model="ignored" :options="ignored_opts" label="Ingored" />
        <q-select
          v-model="submodule"
          :options="submodule_opts"
          label="Ignore submodules"
        />
      </q-card-section>
    </q-card>
  </div>
</template>

<script lang="ts">
import { ref } from "vue";

export default {
  setup() {
    return {
      untracked: ref("Normal"),
      ignored: ref("None"),
      submodule: ref("Traditional"),

      untracked_opts: [
        { label: "No", value: "no" },
        { label: "Normal", value: "normal" },
        { label: "All", value: "all" },
      ],
      ignored_opts: [
        { label: "None", value: "none" },
        { label: "Untracked", value: "untracked" },
        { label: "Dirty", value: "dirty" },
        { label: "All", value: "all" },
      ],
      submodule_opts: [
        { label: "Traditional", value: "traditional" },
        { label: "No", value: "no" },
        { label: "Matching", value: "matching" },
      ],
    };
  },
};
</script>
