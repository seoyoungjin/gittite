<script lang="ts">
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import * as git2rs from "../../api/git2rs";

export default {
  data() {
    return {
      tagAddForm: {
        tagname: null,
        object: null,
        message: null,
        force: false,
      },
      tagListForm: {
        pattern: null,
      },
      tagDeleteForm: {
        tagname: null,
      },
      resTagAdd: null,
      resTagList: null,
      resTagDelete: null,
    };
  },
  components: {
    VueJsonPretty,
  },
  methods: {
    tagAdd() {
      var tagname = this.tagAddForm.tagname;
      var object = this.tagAddForm.object;
      var message = this.tagAddForm.message;
      var force = this.tagAddForm.force;
      git2rs
        .tagAdd(tagname, object, message, force)
        .then((message) => {
          this.resTagAdd = message;
        })
        .catch((e) => {
          if (e) {
            this.resTagAdd = { error: JSON.stringify(e) };
          }
        });
    },

    tagList() {
      var pattern = this.tagListForm.pattern;
      git2rs
        .tagList(pattern)
        .then((message) => {
          this.resTagList = message;
        })
        .catch((e) => {
          if (e) {
            this.resTagList = { error: JSON.stringify(e) };
          }
        });
    },

    tagDelete() {
      var tagname = this.tagDeleteForm.tagname;
      git2rs
        .tagDelete(tagname)
        .then((message) => {
          this.resTagDelete = message;
        })
        .catch((e) => {
          if (e) {
            this.resTagDelete = { error: JSON.stringify(e) };
          }
        });
    },
  },
};
</script>

<template>
  <q-page class="q-ma-lg">
    <h5>Git Tag</h5>

    <!-- add -->
    <h6>Tag Add</h6>
    <q-form id="tag-add">
      <q-input v-model="tagAddForm.tagname" label="Tag Name" />
      <q-input v-model="tagAddForm.object" label="Target Commit Id" />
      <q-input v-model="tagAddForm.message" label="Tag Message" />
      <q-checkbox v-model="tagAddForm.force" label="Force" />
    </q-form>
    <q-btn color="primary" no-caps @click="tagAdd">Add</q-btn>
    <br /><br />

    <div v-if="resTagAdd">
      <vue-json-pretty :data="resTagAdd" />
    </div>
    <br />

    <!-- list -->
    <h6>Tag List</h6>
    <q-form id="tag-list">
      <q-input v-model="tagListForm.pattern" label="Search Pattern" />
    </q-form>
    <q-btn color="primary" no-caps @click="tagList">List</q-btn>
    <br /><br />

    <div v-if="resTagList">
      <vue-json-pretty :data="resTagList" />
    </div>
    <br />

    <!-- delete -->
    <h6>Tag Delete</h6>
    <q-form id="tag-delete">
      <q-input v-model="tagDeleteForm.tagname" label="Tag Name" />
    </q-form>
    <q-btn color="primary" no-caps @click="tagDelete">Delete</q-btn>
    <br /><br />

    <div v-if="resTagDelete">
      <vue-json-pretty :data="resTagDelete" />
    </div>
    <br />

    <!-- usage -->
    <h6>Tag Usage</h6>
    creating a tag
    <pre>
git tag [tagname]
    </pre>

    annotated tag
    <pre>
git tag -a v1.4 -m "my version 1.4"
    </pre>

    list tag
    <pre>
git tag
    </pre>

    tagging old commits
    <pre>
git tag [tagname] [commit hash]
    </pre>

    retagging/replacing old tags
    <pre>
git tag -a -f v1.4 15027957951
    </pre>

    push tags to remote
    <pre>
git push origin v1.4
    </pre>

    checkout tag
    <pre>
git checkout v1.4
    </pre>

    delete tag
    <pre>
git tag -d [tagname]
    </pre>

    filttering tags
    <pre>
git tag -l [pattern]
    </pre>

    remote tag push and delete
    <pre>
git push origin v1.0.3
git push origin --tags
git push origin :v1.0.0
    </pre>
  </q-page>
</template>
