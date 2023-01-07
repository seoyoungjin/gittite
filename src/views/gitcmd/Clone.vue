<template>
  <q-page class="q-ma-lg">
    <h6>Clone</h6>

    <q-card>
      <q-card-section class="row items-center q-pb-none">
        <div class="text-h6">Clone a Repository</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
      </q-card-section>

      <q-separator />

      <q-card-section class="q-pt-none">
        <q-card-actions vertical>
          <q-input v-model="remoteUrl" label="Git URL" />
          <q-input v-model="repositoryName" label="Repository Name" />
          <q-input v-model="localPath" label="Local Path">
            <template v-slot:after>
              <q-btn no-caps @click="selectDirectory"> Choose... </q-btn>
            </template>
          </q-input>
        </q-card-actions>
      </q-card-section>

      <q-separator />

      <q-card-actions align="right">
        <q-btn
          no-caps
          color="primary"
          label="Submit"
          @click="onSubmit"
          :disable="!(remoteUrl && repositoryName)"
        />
        <q-btn no-caps label="Reset" @click="onReset" />
      </q-card-actions>
    </q-card>
    <br />

    <q-card>
      <div class="q-pa-md">
        <RemoteProgress ref="progressRef" />
      </div>
    </q-card>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { open } from "@tauri-apps/api/dialog";
import RemoteProgress from "@/components/RemoteProgress.vue";
import * as git2rs from "@/lib/git2rs";

export default defineComponent({
  components: {
    RemoteProgress,
  },

  data() {
    return {
      remoteUrl: "",
      repositoryName: "",
      localPath: "",
    };
  },

  methods: {
    async selectDirectory() {
      const selected = await open({
        directory: true,
      });
      if (Array.isArray(selected) || selected === null) {
        return;
      }
      this.localPath = selected.split("\\").join("/");
    },
    async cloneRepository() {
      const repositoryPath = this.localPath + "/" + this.repositoryName;
      await git2rs
        .clone(this.remoteUrl, repositoryPath)
        .then((message) => {
          this.$q.notify({
            color: "green-5",
            textColor: "white",
            icon: "cloud",
            message: message,
          });
        })
        .catch((e) => {
          var message = JSON.stringify(e, null, 4);
          this.$q.notify({
            color: "red-5",
            textColor: "white",
            icon: "warning",
            message: message,
          });
        });

      // await git2rs.testProgress();
    },
    onSubmit() {
      (this.$refs.progressRef as any).startProgress();
      this.cloneRepository();
    },
    onReset() {
      this.remoteUrl = "";
      this.repositoryName = "";
      this.localPath = "";

      (this.$refs.progressRef as any).resetProgress();
    },
  },
});
</script>
