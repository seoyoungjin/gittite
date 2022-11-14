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
      branchesInfoForm: {
        local: false,
      },
      branchRemoteForm: {
        name: null,
      },
      checkoutBranchForm: {
        name: null,
      },
      resBranch : null,
      resRenameBranch : null,
      resBranchesInfo : null,
      resBranchRemote : null,
      resCheckoutBranch : null,
      // resCheckoutRemoteBranch : null,
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
      var branchRef = this.renameBranchForm.branchRef;
      var newName = this.renameBranchForm.newName;
      git2rs.renameBranch(branchRef, newName).then((message) => {
        this.resRenameBranch = message;
      }).catch((e) => {
        this.resRenameBranch = { error: JSON.stringify(e) };
      });
    },

    branchesInfo() {
      var local = this.branchesInfoForm.local;
      git2rs.branchesInfo(local).then((message) => {
        this.resBranchesInfo = message;
      }).catch((e) => {
        this.resBranchesInfo = { error: JSON.stringify(e) };
      });
    },

    branchRemote() {
      var name = this.branchRemoteForm.name;
      git2rs.branchRemote(name).then((message) => {
        this.resBranchRemote = message;
      }).catch((e) => {
        this.resBranchRemote = { error: JSON.stringify(e) };
      });
    },

    branchCompareUpstream() {
      var name = this.branchRemoteForm.name;
      git2rs.branchCompareUpstream(name).then((message) => {
        this.resBranchRemote = message;
      }).catch((e) => {
        this.resBranchRemote = { error: JSON.stringify(e) };
      });
    },

    checkoutBranch() {
      var name = this.checkoutBranchForm.name;
      git2rs.checkoutBranch(name).then((message) => {
        this.resCheckoutBranch = message;
      }).catch((e) => {
        this.resCheckoutBranch = { error: JSON.stringify(e) };
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
      <q-input v-model="branchRenameForm.branchRef" label="Branch Ref"
        hint="refs/heads/[branchname]"
      />
      <q-input v-model="branchRenameForm.newName" label="New Name"/>
    </q-form>
    <q-btn color="primary" no-caps @click="renameBranch">Rename</q-btn>
    <br /><br />
    <div>
      <vue-json-pretty :data="resRenameBranch" />
    </div>
    <br />

    <!-- branches info -->
    <h6>Branches Info</h6>
    <q-form id="branches-info">
      <q-checkbox v-model="branchesInfoForm.local" label="Local"/>
    </q-form>
    <q-btn color="primary" no-caps @click="branchesInfo">Branches Info</q-btn>
    <br /><br />
    <div>
      <vue-json-pretty :data="resBranchesInfo" />
    </div>
    <br />

    <!-- branch remote -->
    <h6>Branch Remote/Compare Upstream</h6>
    <q-form id="remote-branch">
      <q-input v-model="branchRemoteForm.name" label="Branch Remote"/>
    </q-form>
    <q-btn color="primary" no-caps @click="branchRemote">Branch Remote</q-btn>
    <q-btn color="primary" no-caps @click="branchCompareUpstream">Compare Upstream</q-btn>
    <br /><br />
    <div>
      <vue-json-pretty :data="resBranchRemote" />
    </div>
    <br />

    <!-- checkout branch -->
    <h6>Checkout Branch</h6>
    <q-form id="checkout-branch">
      <q-input v-model="checkoutBranchForm.name" label="Branch name"/>
    </q-form>
    <q-btn color="primary" no-caps @click="checkoutBranch">Checkout</q-btn>
    <br /><br />
    <div>
      <vue-json-pretty :data="resCheckoutBranch" />
    </div>
    <br />

    <!-- usage -->
    <h6>Branch Usage</h6>

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

    checkout remote branch
    <pre>
git checkout -t origin/[branchname]
    </pre>

    delete remote branch
    <pre>
git push origin --delete [branchname]
    </pre>
  </q-page>
</template>
