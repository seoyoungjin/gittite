<template>
  <q-layout view="hHh lpR fFf" class="bg-grey-1">
    <q-header elevated class="bg-white text-grey-8 q-py-xs" height-hint="58">
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          @click="toggleLeftDrawer"
          aria-label="Menu"
          icon="menu"
        />

        <q-btn flat no-caps no-wrap class="q-ml-xs" v-if="$q.screen.gt.xs">
          <q-icon :name="Gittite" color="red" size="28px" />
          <q-toolbar-title shrink class="text-weight-bold">
            Gittite
          </q-toolbar-title>
        </q-btn>

        <q-space />

        <div class="q-gutter-sm row items-center no-wrap">
          <q-btn round dense flat color="grey-8" icon="apps" v-if="$q.screen.gt.sm">
            <q-tooltip>Apps</q-tooltip>
          </q-btn>
          <q-btn round dense flat color="grey-8" icon="message">
            <q-tooltip>Messages</q-tooltip>
          </q-btn>
        </div>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
      class="bg-grey-2"
      breakpoint="600"
      :width="180"
    >
      <q-scroll-area class="fit">
        <q-list padding>
          <q-item v-for="link in links1" :key="link.text" @click="$router.push(link.link)" v-ripple clickable>
            <q-item-section avatar>
              <q-icon color="grey" :name="link.icon" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ link.text }}</q-item-label>
            </q-item-section>
          </q-item>

          <q-separator class="q-my-md" />

          <q-item v-for="link in links2" :key="link.text" @click="$router.push(link.link)" v-ripple clickable>
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
import { ref } from 'vue'

export default {
  name: 'MyLayout',

  setup () {
    const leftDrawerOpen = ref(false)

    function toggleLeftDrawer () {
      leftDrawerOpen.value = !leftDrawerOpen.value
    }

    return {
      leftDrawerOpen,
      toggleLeftDrawer,

      links1: [
        { icon: 'subscriptions', text: 'Init', link: '/gitcmd/init' },
        { icon: 'subscriptions', text: 'Clone', link: '/gitcmd/clone' },
        { icon: 'home', text: 'Status', link: '/gitcmd/status' },
        { icon: 'restore', text: 'Logs', link: '/gitcmd/logs' },
        { icon: 'folder', text: 'App/Remove', link: '/gitcmd/addremove' },
        { icon: 'star_border', text: 'Branch', link: '/gitcmd/branch' },
        { icon: 'whatshot', text: 'Tags', link: '/gitcmd/tags' },
        { icon: 'watch_later', text: 'Stash', link: '/gitcmd/stash' },
        { icon: 'thumb_up_alt', text: 'Remote', link: '/gitcmd/remote' }
      ],
      links2: [
        { icon: 'settings', text: 'Settings', link: '/gitcmd/settings' },
      ],
      buttons1: [
        { text: 'About', link: '/about' },
      ],
      buttons2: [
        { text: 'Test features', link: '/test' }
      ]
    }
  }
}
</script>

<style>
.YL__drawer-footer-link {
  color: inherit;
  text-decoration: none;
  font-weight: 500;
  font-size: .75rem;
}

.YL__drawer-footer-link:hover {
    color: #000;
}
</style>
