import { defineStore } from "pinia";
import * as git2rs from "@/lib/git2rs";
import type { CommitData, CommitInfo } from "@/models/commit";
import { Commit } from "@/models/commit";
import type { StatusItem } from "@/models/status";
import { useRepositoryStore } from "./repository";

const CommitBatchSize = 100;
const LoadingHistoryRequestKey = "history";

const requestsInFight = new Set<string>();

export const useHistoryStore = defineStore("history", {
  state: () => {
    return {
      commitList: [] as Commit[],
      // current item
      currentItem: null as Commit | null,
      currentCommitInfo: null as CommitInfo | null,
      currentCommitFiles: [] as StatusItem[],
    };
  },

  getters: {
    commitLogs: (state) => state.commitList,
    // current item
    historyCurrent: (state) => state.currentItem,
    historyCommitInfo: (state) => state.currentCommitInfo,
    historyCommitFiles: (state) => state.currentCommitFiles,
  },

  actions: {
    resetHistory() {
      this.commitList = [];
      this.currentItem = null;
      this.currentCommitInfo = null;
      this.currentCommitFiles = [] as StatusItem[];
    },

    // Load a batch of commits from the repository,
    // using a given commitish object as the starting point
    async loadCommitBatch(commitish: string, skip: number): Promise<any> {
      if (requestsInFight.has(LoadingHistoryRequestKey)) {
        return null;
      }

      const requestKey = `history/compare/${commitish}/skip/${skip}`;
      if (requestsInFight.has(requestKey)) {
        return null;
      }

      requestsInFight.add(requestKey);

      const commits = await git2rs
        .getCommits(commitish, CommitBatchSize, skip)
        .catch(() => {
          return [];
        });

      requestsInFight.delete(requestKey);
      if (!commits) {
        return null;
      }

      this.storeCommits_(commits);
      return commits.map((c) => c.commit_id);
    },

    async loadNextCommitBatch(): Promise<any> {
      return await this.loadCommitBatch("HEAD", this.commitList.length);
    },

    storeCommits_(newCommits: CommitData[]) {
      const repoStore = useRepositoryStore();
      for (const item of newCommits) {
        const commit = new Commit(item);
        if (repoStore.allTags.has(item.commit_id)) {
          commit.tags = repoStore.allTags.get(item.commit_id);
        }
        this.commitList.push(commit);
      }
      // select first item at start
      if (!this.currentItem && this.commitList.length) {
        this.setCurrentItem(this.commitList[0]);
      }
    },

    // current item
    setCurrentItem(current: Commit) {
      this.currentItem = current;
      this.loadCommitInfoAndFiles(current.commit_id);
    },

    async loadCommitInfoAndFiles(commitId: string) {
      this.currentCommitInfo = await git2rs.commitInfo(commitId);
      this.currentCommitFiles = await git2rs.commitFiles(commitId);
    },
  },
});
