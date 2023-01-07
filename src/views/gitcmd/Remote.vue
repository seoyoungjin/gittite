<template>
  <q-page class="q-ma-lg">
    <h6>Remote</h6>

    <pre>
$ git remote -v
origin   https://github.com/... (fetch)
origin   https://github.com/... (push)
    </pre>

    <q-btn color="primary" no-caps @click="gitFetch()">Fetch</q-btn>
    <br />
    <br />
    <div v-if="resFetch">
      <vue-json-pretty :data="resFetch" />
    </div>

    <q-btn color="primary" no-caps @click="gitPush()">Push</q-btn>
    <br />
    <br />
    <div v-if="resPush">
      <vue-json-pretty :data="resPush" />
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "vue-json-pretty/lib/styles.css";
import VueJsonPretty from "vue-json-pretty";
import * as git2rs from "../../lib/git2rs";

export default defineComponent({
  components: {
    VueJsonPretty,
  },

  data() {
    return {
      resFetch: null,
      resPush: null,
    };
  },

  methods: {
    gitFetch() {
      git2rs
        .fetch()
        .then((message) => {
          this.resFetch = message as any;
        })
        .catch((e) => {
          this.resFetch = { error: JSON.stringify(e) } as any;
        });
    },

    gitPush() {
      git2rs
        .push()
        .then((message) => {
          this.resFetch = message as any;
        })
        .catch((e) => {
          if (e) {
            this.resFetch = { error: JSON.stringify(e) } as any;
          }
        });
    },
  },
});
</script>
