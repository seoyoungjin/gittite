<template>
  <div class="q-ma-none">
    <!--Unstaged-->
    <div class="text-h7">Unstaged Changes</div>

    <q-scroll-area style="height: 400px">
      <q-list dense bordered padding class="rounded-borders">
        <q-item
          v-for="(item, index) in unstagedData"
          :key="index"
          clickable
          v-ripple
        >
          <q-item-section side>
            <q-icon name="folder" color="amber" />
            <!-- {{ item.wtree }} -->
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ item.path }}</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>
    </q-scroll-area>

    <q-list dense bordered padding class="rounded-borders">
      <q-item clickable v-ripple>
        <q-item-section> Staged </q-item-section>
      </q-item>

      <q-item
        v-for="(item, index) in stagedData"
        :key="index"
        clickable
        v-ripple
      >
        <q-item-section side>
          <q-icon name="folder" color="blue" />
          <!-- {{ item.stage }} -->
        </q-item-section>
        <q-item-section>
          <q-item-label>{{ item.path }}</q-item-label>
        </q-item-section>
      </q-item>
    </q-list>

    <div>
      <ChangesOption />
    </div>
  </div>
</template>

<script lang="ts">
import ChangesOption from "@/components/ChangesOption.vue";
import * as git2rs from "@/api/git2rs";

export default {
  components: {
    ChangesOption,
  },

  mounted() {
    this.refreshStatus();
  },

  data() {
    return {
      stagedData: [],
      unstagedData: [],
    };
  },

  methods: {
    refreshStatus() {
      (async () => {
        this.stagedData = await git2rs.getStatus("stage");
        this.unstagedData = await git2rs.getStatus("workdir");
      })();
    },
  },
};
</script>
