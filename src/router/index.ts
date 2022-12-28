import { createRouter, createWebHistory } from "vue-router";
import WorkSpace from "../views/WorkSpace.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "workspace",
      component: WorkSpace,
    },
    {
      path: "/select",
      name: "selectRepository",
      component: () => import("../views/SelectRepository.vue"),
    },
    {
      path: "/gitcmd",
      name: "gitcmd",
      component: () => import("../layouts/DevLayout.vue"),
      children: [
        {
          path: "home",
          name: "home",
          component: () => import("../views/HomeView.vue"),
        },
        {
          path: "repo",
          name: "repository",
          component: () => import("../views/gitcmd/Repo.vue"),
        },
        {
          path: "init",
          name: "git-init",
          component: () => import("../views/gitcmd/Init.vue"),
        },
        {
          path: "clone",
          name: "git-clone",
          component: () => import("../views/gitcmd/Clone.vue"),
        },
        {
          path: "status",
          name: "git-tatus",
          component: () => import("../views/gitcmd/Status.vue"),
        },
        {
          path: "logs",
          name: "git-logs",
          component: () => import("../views/gitcmd/Logs.vue"),
        },
        {
          path: "commit",
          name: "git-commit",
          component: () => import("../views/gitcmd/Commit.vue"),
        },
        {
          path: "diff",
          name: "git-diff",
          component: () => import("../views/gitcmd/Diff.vue"),
        },
        {
          path: "addremove",
          name: "git-addremove",
          component: () => import("../views/gitcmd/AddRemove.vue"),
        },
        {
          path: "branch",
          name: "git-branch",
          component: () => import("../views/gitcmd/Branch.vue"),
        },
        {
          path: "tags",
          name: "git-tags",
          component: () => import("../views/gitcmd/Tags.vue"),
        },
        {
          path: "stash",
          name: "git-stash",
          component: () => import("../views/gitcmd/Stash.vue"),
        },
        {
          path: "addremove",
          name: "git-add-remove",
          component: () => import("../views/gitcmd/AddRemove.vue"),
        },
        {
          path: "remote",
          name: "git-remote",
          component: () => import("../views/gitcmd/Remote.vue"),
        },
        {
          path: "blame",
          name: "git-blame",
          component: () => import("../views/gitcmd/Blame.vue"),
        },
        {
          path: "settings",
          name: "git-settings",
          component: () => import("../views/gitcmd/Settings.vue"),
        },
        {
          path: "about",
          name: "tite-about",
          component: () => import("../views/AboutView.vue"),
        },
        {
          path: "test",
          name: "test",
          component: () => import("../views/Test.vue"),
        },
      ],
    },
  ],
});

export default router;
