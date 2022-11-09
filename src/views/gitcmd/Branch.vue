<script lang="ts">
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import * as git2rs from '../../api/git2rs';

export default {
  data() {
    return {
      branchForm: {
        name: null,
      },
      branchRenameForm: {
        branchRef: null,
        newName: null,
      },
      resBranch : null,
      resRenameBranch : null,
      resGetBranchesInfo : null,
      resGetBranchRemote : null,
      resCheckoutBranch : null,
      resCheckoutRemoteBranch : null,
      resBranchCompareUpstream : null,
    };
  },
  components: {
    VueJsonPretty,
  },
  methods: {
    createBranch() {
      var name = this.branchForm.name;
      git2rs.createBranch(name).then((message) => {
        this.resBranch = message;
      }).catch((e) => {
        this.resBranch = { error: JSON.stringify(e) };
      });
    },

    deleteBranch() {
      var name = this.branchForm.name;
      git2rs.deleteBranch(name).then((message) => {
        this.resBranch = message;
      }).catch((e) => {
        this.resBranch = { error: JSON.stringify(e) };
      });
    },

    renameBranch() {
      alert(newName);
      var branchRef = this.branchForm.branchRef;
      var newName = this.branchForm.newName;
      git2rs.renameBranch(branchRef, newName).then((message) => {
        this.resBranch = message;
      }).catch((e) => {
        this.resBranch = { error: JSON.stringify(e) };
      });
    },
  },
};
</script>

<template>
  <q-page class="q-ma-lg">
    <h5>Git Branch</h5>

    <!-- create/delete -->
    <h6>Branch Create/Delete</h6>
    <q-form id="branch-create-delete">
      <q-input v-model="branchForm.name" label="Branch Name"/>
    </q-form>
    <q-btn color="primary" no-caps @click="createBranch">Create</q-btn>
    <q-btn color="primary" no-caps @click="deleteBranch">Delete</q-btn>
    <br /><br />
    <div>
      <vue-json-pretty :data="resBranch" />
    </div>
    <br />

    <!-- nename -->
    <h6>Branch Rename</h6>
    <q-form id="branch-rename">
      <q-input v-model="branchRenameForm.branchRef" label="Branch Ref"/>
      <q-input v-model="branchRenameForm.newName" label="New Name"/>
    </q-form>
    <q-btn color="primary" no-caps @click="branchRename">Rename</q-btn>
    <br /><br />
    <div>
      <vue-json-pretty :data="resRenameBranch" />
    </div>
    <br />

    create a branch
    <pre>
git branch [branchname]
    </pre>

    list branches
    <pre>
git branch
git branch -a
    </pre>

    delete branches
    <pre>
git branch -d [branchname]
git branch -D [branchname]
    </pre>

    checkout
    <pre>
git checkout -b [branchname]
    </pre>
  </q-page>
</template>
