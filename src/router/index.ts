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
      path: "/about",
      name: "about",
      component: () => import("../views/AboutView.vue"),
    },
    {
      path: "/test",
      name: "test",
      component: () => import("../views/Test.vue"),
    },
    {
      path: "/gitcmd",
      name: "gitcmd",
      component: () => import("../views/HomeView.vue"),
    },
    {
      path: "/gitcmd/repo",
      name: "repository",
      component: () => import("../views/gitcmd/Repo.vue"),
    },
    {
      path: "/gitcmd/init",
      name: "git-init",
      component: () => import("../views/gitcmd/Init.vue"),
    },
    {
      path: "/gitcmd/clone",
      name: "git-clone",
      component: () => import("../views/gitcmd/Clone.vue"),
    },
    {
      path: "/gitcmd/status",
      name: "git-tatus",
      component: () => import("../views/gitcmd/Status.vue"),
    },
    {
      path: "/gitcmd/logs",
      name: "git-logs",
      component: () => import("../views/gitcmd/Logs.vue"),
    },
    {
      path: "/gitcmd/commit",
      name: "git-commit",
      component: () => import("../views/gitcmd/Commit.vue"),
    },
    {
      path: "/gitcmd/diff",
      name: "git-diff",
      component: () => import("../views/gitcmd/Diff.vue"),
    },
    {
      path: "/gitcmd/addremove",
      name: "git-addremove",
      component: () => import("../views/gitcmd/AddRemove.vue"),
    },
    {
      path: "/gitcmd/branch",
      name: "git-branch",
      component: () => import("../views/gitcmd/Branch.vue"),
    },
    {
      path: "/gitcmd/tags",
      name: "git-tags",
      component: () => import("../views/gitcmd/Tags.vue"),
    },
    {
      path: "/gitcmd/stash",
      name: "git-stash",
      component: () => import("../views/gitcmd/Stash.vue"),
    },
    {
      path: "/gitcmd/addremove",
      name: "git-add-remove",
      component: () => import("../views/gitcmd/AddRemove.vue"),
    },
    {
      path: "/gitcmd/remote",
      name: "git-remote",
      component: () => import("../views/gitcmd/Remote.vue"),
    },
    {
      path: "/gitcmd/blame",
      name: "git-blame",
      component: () => import("../views/gitcmd/Blame.vue"),
    },
    {
      path: "/gitcmd/settings",
      name: "git-settings",
      component: () => import("../views/gitcmd/Settings.vue"),
    },
  ],
});

export default router;
