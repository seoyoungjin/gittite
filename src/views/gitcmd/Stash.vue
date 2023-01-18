<script lang="ts">
import { defineComponent } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import { invoke } from "@tauri-apps/api/tauri";
import * as git2rs from "../../lib/git2rs";

export default defineComponent({
  data() {
    return {
      stashIdList: [],

      stashSaveForm: {
        message: "",
        includeUntracked: false,
        keepIndex: false,
      },
      stashForm: {
        stashid: "",
      },
      resStashSave: null,
      resStashList: null as any,
      resStashDPA: null,
    };
  },

  components: {
    VueJsonPretty,
  },

  async mounted() {
    await this.stashList();
  },

  methods: {
    async stashList() {
      // to show enum serialization, use invoke() instead of git2rs.stashList()
      this.resStashList = await invoke("stash", { args: ["list"] }).catch(
        (e) => {
          alert(JSON.stringify(e, null, 4));
          return [];
        }
      );
      // decapsulation enum
      if ("StashList" in this.resStashList) {
        this.stashIdList = this.resStashList.StashList.map((x: any) => { return x.id })
      } else {
        this.stashIdList = this.resStashList.map((x: any) => { return x.id })
      }
    },

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
          this.resStashSave = { kkerror: JSON.stringify(e) } as any;
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

    <!-- list -->
    <q-btn color="primary" no-caps @click="stashList">Stash List</q-btn>
    <br /><br />

    <div v-if="resStashList">
      <vue-json-pretty :data="resStashList" />
    </div>
    <br />

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

    <!-- drop/pop/apply -->
    <q-select
      v-model="stashForm.stashid"
      :options="stashIdList"
      label="Stash ID"
    />
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
