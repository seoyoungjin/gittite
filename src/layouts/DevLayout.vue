<template>
  <q-layout view="hHh lpR fFf" class="bg-grey-1">
    <ToolBar @showLeftDrawer="showLeftDrawer" />

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
      class="bg-grey-2"
      breakpoint="600"
      :width="200"
    >
      <q-scroll-area class="fit">
        <q-list padding>
          <q-item
            v-for="link in links1"
            :key="link.text"
            @click="$router.push(link.link)"
            v-ripple
            clickable
          >
            <q-item-section avatar>
              <q-icon color="grey" :name="link.icon" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ link.text }}</q-item-label>
            </q-item-section>
          </q-item>

          <q-separator class="q-my-md" />

          <q-item
            v-for="link in links2"
            :key="link.text"
            @click="$router.push(link.link)"
            v-ripple
            clickable
          >
            <q-item-section avatar>
              <q-icon color="grey" :name="link.icon" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ link.text }}</q-item-label>
            </q-item-section>
          </q-item>

          <q-separator class="q-mt-md q-mb-lg" />

          <div class="q-px-md text-grey-9">
            <div class="row items-center q-gutter-x-sm q-gutter-y-xs">
              <a
                v-for="button in buttons1"
                :key="button.text"
                class="YL__drawer-footer-link"
                href="javascript:void(0)"
                @click="$router.push(button.link)"
              >
                {{ button.text }}
              </a>
            </div>
          </div>
          <div class="q-py-md q-px-md text-grey-9">
            <div class="row items-center q-gutter-x-sm q-gutter-y-xs">
              <a
                v-for="button in buttons2"
                :key="button.text"
                class="YL__drawer-footer-link"
                href="javascript:void(0)"
                @click="$router.push(button.link)"
              >
                {{ button.text }}
              </a>
            </div>
          </div>
        </q-list>
      </q-scroll-area>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { ref } from "vue";
import ToolBar from "@/components/toolbar/dev/ToolBar.vue";

export default {
  name: "DevLayout",

  setup() {
    const leftDrawerOpen = ref(true);

    function showLeftDrawer(value: boolean) {
      leftDrawerOpen.value = value;
    }

    return {
      leftDrawerOpen,
      showLeftDrawer,

      links1: [
        { icon: "home", text: "Home", link: "/gitcmd/home" },
        { icon: "subscriptions", text: "Repository", link: "/gitcmd/repo" },
        { icon: "list", text: "Init", link: "/gitcmd/init" },
        { icon: "file_copy", text: "Clone", link: "/gitcmd/clone" },
        { icon: "more", text: "Status", link: "/gitcmd/status" },
        { icon: "restore", text: "Logs", link: "/gitcmd/logs" },
        { icon: "add", text: "Commit", link: "/gitcmd/commit" },
        { icon: "difference", text: "Diff", link: "/gitcmd/diff" },
        { icon: "folder", text: "Stage", link: "/gitcmd/stage" },
        { icon: "star_border", text: "Branch", link: "/gitcmd/branch" },
        { icon: "whatshot", text: "Tags", link: "/gitcmd/tags" },
        { icon: "watch_later", text: "Stash", link: "/gitcmd/stash" },
        { icon: "wifi", text: "Remote", link: "/gitcmd/remote" },
        { icon: "thumb_up_alt", text: "Blame", link: "/gitcmd/blame" },
      ],
      links2: [
        { icon: "settings", text: "Settings", link: "/gitcmd/settings" },
      ],
      buttons1: [{ text: "About", link: "/gitcmd/about" }],
      buttons2: [{ text: "Test features", link: "/gitcmd/test" }],
    };
  },

  components: {
    ToolBar,
  },
};
</script>

<style>
.YL__drawer-footer-link {
  color: inherit;
  text-decoration: none;
  font-weight: 500;
  font-size: 0.75rem;
}

.YL__drawer-footer-link:hover {
  color: #000;
}
</style>
