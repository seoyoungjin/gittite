import { describe, it, expect, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useRepositoryStore } from "@/stores/repository";

import { mockIPC } from "@tauri-apps/api/mocks";
// import { invoke } from "@tauri-apps/api/tauri";
import { TestRepoIPC } from "../fixtures/test-repo";
// import TestRepoIPC from "../fixtures/test-repo.json";

describe("Repository Store", () => {
  let store = null;

  mockIPC((cmd, args) => {
    console.log(">> ", cmd, args);
    if (Object.keys(TestRepoIPC).includes(cmd)) {
      const item = TestRepoIPC[cmd];
      if (typeof item == "function") {
        return item(args);
      } else {
        return TestRepoIPC[cmd];
      }
    } else {
      console.log(">>>> ", cmd, args);
      return cmd;
    }
  });

  beforeEach(async () => {
    setActivePinia(createPinia());

    store = useRepositoryStore();
    await store.setRepository(".");
  });

  it("test setRepository", async () => {
    expect(store.repositoryPath).toBe("/tmp/test_repo");
    expect(store.repositoryName).toBe("test_repo");
    expect(store.currentBranch).toBe("master");

    expect(store.allBranches.length).toBe(4);
    expect(store.getBranchInfo("develop")).toStrictEqual({
      name: "develop",
      reference: "refs/heads/develop",
      top_commit: "fa01b51116fbe5b6a7a01e9741a26b7110656cab",
      top_commit_message: "refresh button added",
      details: {
        Local: {
          has_upstream: true,
          is_head: false,
          remote: "origin",
        },
      },
    });
  });
});
