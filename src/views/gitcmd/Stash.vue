<script lang="ts">
import { defineComponent } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import * as git2rs from "../../api/git2rs";

export default defineComponent({
  data() {
    return {
      stashSaveForm: {
        message: "",
        includeUntracked: false,
        keepIndex: false,
      },
      stashForm: {
        stashid: "",
      },
      resStashSave: null,
      resStashList: null,
      resStashDPA: null,
    };
  },

  components: {
    VueJsonPretty,
  },

  methods: {
    stashSave() {
      var message = this.stashSaveForm.message;
      var untracked = this.stashSaveForm.includeUntracked;
      var keepIndex = this.stashSaveForm.keepIndex;
      git2rs
        .stashSave(message, untracked, keepIndex)
        .then((message) => {
          this.resStashSave = message as any;
        })
        .catch((e) => {
          this.resStashSave = { error: JSON.stringify(e) } as any;
        });
    },

    stashList() {
      git2rs
        .stashList()
        .then((message) => {
          this.resStashList = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resStashList = { error: JSON.stringify(e) } as any;
          }
        });
    },

    stashDrop() {
      var stashid = this.stashForm.stashid;
      git2rs
        .stashDrop(stashid)
        .then((message) => {
          this.resStashDPA = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resStashDPA = { error: JSON.stringify(e) } as any;
          }
        });
    },

    stashPop() {
      var stashid = this.stashForm.stashid;
      git2rs
        .stashPop(stashid)
        .then((message) => {
          this.resStashDPA = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resStashDPA = { error: JSON.stringify(e) } as any;
          }
        });
    },

    stashApply() {
      var stashid = this.stashForm.stashid;
      git2rs
        .stashApply(stashid)
        .then((message) => {
          this.resStashDPA = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resStashDPA = { error: JSON.stringify(e) } as any;
          }
        });
    },
  },
});
</script>

<template>
  <q-page class="q-ma-lg">
    <h5>Git Stash</h5>

    <!-- save -->
    <q-form v-on:submit="stashSave" id="stash-save">
      <q-input v-model="stashSaveForm.message" label="Stash Message" />
      <q-checkbox
        v-model="stashSaveForm.includeUntracked"
        label="Stash Untracked"
      />
      <q-checkbox v-model="stashSaveForm.keepIndex" label="Keep Index" />
    </q-form>
    <q-btn color="primary" no-caps @click="stashSave">Stash Save</q-btn>
    <br /><br />

    <div v-if="resStashSave">
      <vue-json-pretty :data="resStashSave" />
    </div>
    <br />

    <!-- list -->
    <q-btn color="primary" no-caps @click="stashList">Stash List</q-btn>
    <br /><br />

    <div v-if="resStashList">
      <vue-json-pretty :data="resStashList" />
    </div>
    <br />

    <!-- drop/pop/apply -->
    <q-form id="stash-form">
      <q-input v-model="stashForm.stashid" label="Stash ID" />
    </q-form>
    <div>
      <q-btn color="primary" no-caps @click="stashDrop">Drop</q-btn>
      <q-btn color="primary" no-caps @click="stashPop">Pop</q-btn>
      <q-btn color="primary" no-caps @click="stashApply">Apply</q-btn>
    </div>
    <br /><br />

    <div v-if="resStashDPA">
      <vue-json-pretty :data="resStashDPA" />
    </div>
    <br />

    <!-- usage  -->
    Stash the changes in a dirty working directory away stashing your work
    <pre>
git stash
git stash save [message] [--keep-index | -k] [--include-untracked | -u]
    </pre>

    list
    <pre>
git stash list
    </pre>

    view stash diff
    <pre>
git stash show
git stash show -p
    </pre>

    creating a branch from your stash
    <pre>
git stash branch [branchnamel [stash]
    </pre>

    cleaning up your stash
    <pre>
git stash drop [stash]
git stash clear
    </pre>

    re-applying your stashed changes
    <pre>
git stash [pop | apply] [stash]
    </pre>
  </q-page>
</template>
