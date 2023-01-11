<template>
  <div class="column" style="height: 100%">
    <div class="col-auto">{{ commitInfo.message.subject }}</div>
    <div class="col-auto bg-grey-2" v-if="commitInfo.message.body">
      {{ commitInfo.message.body }}
    </div>
    <div class="col">
      <q-splitter
        v-model="splitterModel"
        :limits="[100, 300]"
        unit="px"
        before-class="overflow-hidden"
        after-class="overflow-hidden"
        style="height: 100%"
      >
        <template v-slot:before>
          <CommitFileList
            :commitFiles="commitFiles"
            @selectItem="handleSelectItem"
          />
        </template>

        <template v-slot:after>
          <DiffView :selection="selection" :selectedFile="selectedFile" />
        </template>
      </q-splitter>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import CommitFileList from "@/components/history/CommitFileList.vue";
import DiffView from "@/components/DiffView.vue";
import type { CommitInfo } from "@/models/commit";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  name: "ContentForHistory",

  setup() {
    return {
      splitterModel: ref(250),
    };
  },

  props: {
    selection: null,
  },

  watch: {
    selection: function (value) {
      // TODO: move to store for test
      // alert(JSON.stringify(value));
      this.getCommitInfo(value.commit_id);
      this.getCommitFiles(value.commit_id);
      // alert(JSON.stringify(this.commitFiles));
    },
  },

  components: {
    CommitFileList,
    DiffView,
  },

  data() {
    return {
      commitInfo: { message: { subject: "", body: null } } as CommitInfo,
      commitFiles: [],
      selectedFile: null,
    };
  },

  methods: {
    async getCommitInfo(commitId: string) {
      this.commitInfo = await git2rs.commitInfo(commitId);
    },

    async getCommitFiles(commitId: string) {
      this.commitFiles = (await git2rs.commitFiles(commitId)) as any;
    },

    handleSelectItem(item: any) {
      this.selectedFile = item.path;
    },
  },
});
</script>
