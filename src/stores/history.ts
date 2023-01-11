import { defineStore } from "pinia";
import * as git2rs from "@/lib/git2rs";
import type { CommitData } from "@/models/commit";

const CommitBatchSize = 100;
const LoadingHistoryRequestKey = "history";

const requestsInFight = new Set<string>();

export const useHistoryStore = defineStore("history", {
  state: () => {
    return {
      currentItem: null as CommitData | null,
      commitList: [] as CommitData[],
    };
  },

  getters: {
    historyCurrent: (state) => state.currentItem,
    commitLogs: (state) => state.commitList,
  },

  actions: {
    // Load a batch of commits from the repository,
    // using a given commitish object as the starting point
    async loadCommitBatch(commitish: string, skip: number): Promise<any> {
      if (skip === 0) {
        this.commitList = [];
      }

      if (requestsInFight.has(LoadingHistoryRequestKey)) {
        return null;
      }

      const requestKey = `history/compare/${commitish}/skip/${skip}`;
      if (requestsInFight.has(requestKey)) {
        return null;
      }

      requestsInFight.add(requestKey);

      const commits = await git2rs.getCommits(commitish, CommitBatchSize, skip);

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
      this.commitList = this.commitList.concat(newCommits);
      /*
      for (cosnt commit of commits) {
        this.commitLookup.set(commit.sha, commit)
      }
      */
    },

    setCurrentItem(current: CommitData) {
      this.currentItem = current;
    },
  },
});
