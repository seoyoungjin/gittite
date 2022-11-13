<script setup lang="ts">
import WelcomeItem from "./WelcomeItem.vue";

import CommunityIcon from "./icons/IconCommunity.vue";
import ToolingIcon from "./icons/IconTooling.vue";
import DocumentationIcon from "./icons/IconDocumentation.vue";
import EcosystemIcon from "./icons/IconEcosystem.vue";
import SupportIcon from "./icons/IconSupport.vue";
</script>

<template>
  <WelcomeItem>
    <template #icon>
      <CommunityIcon />
    </template>
    <template #heading></template>

    This is the place to test git function one by one. The return of each
    command can be viewed directly in json format or in a different layout.
  </WelcomeItem>

  <WelcomeItem>
    <template #icon>
      <ToolingIcon />
    </template>
    <template #heading>Tooling</template>

    This project is based on tauri framework and <code>git2-rs</code> library.
    And most of code is copied from Gitui project.
    <ul>
      <li>
        <a href="https://tauri.app/" target="_blank" rel="noopener">Tauri</a>
      </li>
      <li>
        <a
          href="https://github.com/extrawurst/gitui"
          target="_blank"
          rel="noopener"
          >Gitui</a
        >
      </li>
    </ul>
    User interface is written with <code>Vue.js</code> and <code>Quasar</code>.
    <ul>
      <li>
        <a href="https://vuejs.org/" target="_blank" rel="noopener">Vue</a>,
      </li>
      <li>
        <a href="https://quasar.dev/" target="_blank" rel="noopener">Quasar</a>,
      </li>
    </ul>
  </WelcomeItem>

  <WelcomeItem>
    <template #icon>
      <DocumentationIcon />
    </template>
    <template #heading>Documentaion</template>
    More instructions are available in <code>README.md</code>
  </WelcomeItem>

  <WelcomeItem>
    <template #icon>
      <EcosystemIcon />
    </template>
    <template #heading>Test</template>

    The test consists of a rust code unit test and a javascript case test.
    <br />
    Visit <code>test features</code> page.
  </WelcomeItem>

  <WelcomeItem>
    <template #icon>
      <SupportIcon />
    </template>
    <template #heading>Support</template>

    Any help is welcome.
  </WelcomeItem>
</template>
