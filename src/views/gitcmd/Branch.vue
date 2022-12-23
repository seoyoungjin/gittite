<script lang="ts">
import { defineComponent } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import * as git2rs from "../../api/git2rs";

export default defineComponent({
  data() {
    return {
      branchForm: {
        name: "",
      },
      branchRenameForm: {
        branchRef: "",
        newName: "",
      },
      branchesInfoForm: {
        local: false,
      },
      branchRemoteForm: {
        name: "",
      },
      checkoutBranchForm: {
        name: "",
      },
      checkoutRemoteBranchForm: {
        name: "",
      },
      resBranch: null,
      resRenameBranch: null,
      resBranchesInfo: null,
      resBranchRemote: null,
      resCheckoutBranch: null,
      resCheckoutRemoteBranch: null,
    };
  },
  components: {
    VueJsonPretty,
  },
  methods: {
    createBranch() {
      var name = this.branchForm.name;
      git2rs
        .createBranch(name)
        .then((message) => {
          this.resBranch = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resBranch = { error: JSON.stringify(e) } as any;
          }
        });
    },

    deleteBranch() {
      var name = this.branchForm.name;
      git2rs
        .deleteBranch(name)
        .then((message) => {
          this.resBranch = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resBranch = { error: JSON.stringify(e) } as any;
          }
        });
    },

    renameBranch() {
      var branchRef = this.branchRenameForm.branchRef;
      var newName = this.branchRenameForm.newName;
      git2rs
        .renameBranch(branchRef, newName)
        .then((message) => {
          this.resRenameBranch = message as any;
        })
        .catch((e) => {
          this.resRenameBranch = { error: JSON.stringify(e) } as any;
        });
    },

    branchesInfo() {
      var local = this.branchesInfoForm.local;
      git2rs
        .branchesInfo(local)
        .then((message) => {
          this.resBranchesInfo = message as any;
        })
        .catch((e) => {
          this.resBranchesInfo = { error: JSON.stringify(e) } as any;
        });
    },

    branchRemote() {
      var name = this.branchRemoteForm.name;
      git2rs
        .branchRemote(name)
        .then((message) => {
          this.resBranchRemote = message as any;
        })
        .catch((e) => {
          this.resBranchRemote = { error: JSON.stringify(e) } as any;
        });
    },

    branchCompareUpstream() {
      var name = this.branchRemoteForm.name;
      git2rs
        .branchCompareUpstream(name)
        .then((message) => {
          this.resBranchRemote = message as any;
        })
        .catch((e) => {
          this.resBranchRemote = { error: JSON.stringify(e) } as any;
        });
    },

    checkoutBranch() {
      var name = this.checkoutBranchForm.name;
      git2rs
        .checkoutBranch(name)
        .then((message) => {
          this.resCheckoutBranch = message as any;
        })
        .catch((e) => {
          this.resCheckoutBranch = { error: JSON.stringify(e) } as any;
        });
    },

    checkoutRemoteBranch() {
      var name = this.checkoutRemoteBranchForm.name;
      git2rs
        .checkoutRemoteBranch(name)
        .then((message) => {
          this.resCheckoutRemoteBranch = message as any;
        })
        .catch((e) => {
          this.resCheckoutRemoteBranch = { error: JSON.stringify(e) } as any;
        });
    },
  },
});
</script>

<template>
  <q-page class="q-ma-lg">
    <h5>Git Branch</h5>

    <!-- create/delete -->
    <h6>Branch Create/Delete</h6>
    <q-form id="branch-create-delete">
      <q-input v-model="branchForm.name" label="Branch Name" />
    </q-form>
    <q-btn color="primary" no-caps @click="createBranch">Create</q-btn>
    <q-btn color="primary" no-caps @click="deleteBranch">Delete</q-btn>
    <br /><br />
    <div v-if="resBranch">
      <vue-json-pretty :data="resBranch" />
    </div>
    <br />

    <!-- nename -->
    <h6>Branch Rename</h6>
    <q-form id="branch-rename">
      <q-input
        v-model="branchRenameForm.branchRef"
        label="Branch Ref"
        hint="refs/heads/[branchname]"
      />
      <q-input v-model="branchRenameForm.newName" label="New Name" />
    </q-form>
    <q-btn color="primary" no-caps @click="renameBranch">Rename</q-btn>
    <br /><br />
    <div v-if="resRenameBranch">
      <vue-json-pretty :data="resRenameBranch" />
    </div>
    <br />

    <!-- branches info -->
    <h6>Branches Info</h6>
    <q-form id="branches-info">
      <q-checkbox v-model="branchesInfoForm.local" label="Local" />
    </q-form>
    <q-btn color="primary" no-caps @click="branchesInfo">Branches Info</q-btn>
    <br /><br />
    <div v-if="resBranchesInfo">
      <vue-json-pretty :data="resBranchesInfo" />
    </div>
    <br />

    <!-- branch remote -->
    <h6>Branch Remote/Compare Upstream</h6>
    <q-form id="remote-branch">
      <q-input v-model="branchRemoteForm.name" label="Branch Remote" />
    </q-form>
    <q-btn color="primary" no-caps @click="branchRemote">Branch Remote</q-btn>
    <q-btn color="primary" no-caps @click="branchCompareUpstream"
      >Compare Upstream</q-btn
    >
    <br /><br />
    <div v-if="resBranchRemote">
      <vue-json-pretty :data="resBranchRemote" />
    </div>
    <br />

    <!-- checkout branch -->
    <h6>Checkout Branch</h6>
    <q-form id="checkout-branch">
      <q-input v-model="checkoutBranchForm.name" label="Branch name" />
    </q-form>
    <q-btn color="primary" no-caps @click="checkoutBranch">Checkout</q-btn>
    <br /><br />
    <div v-if="resCheckoutBranch">
      <vue-json-pretty :data="resCheckoutBranch" />
    </div>
    <br />

    <!-- checkout remote branch -->
    <h6>Checkout Remote Branch</h6>
    <q-form id="checkout-remote-branch">
      <q-input v-model="checkoutRemoteBranchForm.name" label="Branch name" />
    </q-form>
    <q-btn color="primary" no-caps @click="checkoutRemoteBranch"
      >Checkout</q-btn
    >
    <br /><br />
    <div v-if="resCheckoutRemoteBranch">
      <vue-json-pretty :data="resCheckoutRemoteBranch" />
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
