import { describe, it, expect, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import { validateBranchName } from "@/lib/validateBranchName";

import { mockIPC } from "@tauri-apps/api/mocks";
import { TestRepoIPC } from "../fixtures/test-repo";

describe("Create Branch", () => {
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

  beforeEach(() => {
    setActivePinia(createPinia());

    store = useRepositoryStore();
  });

  it("test branch exists", async () => {
    await store.setRepository(".");

    expect(store.allBranches.findIndex((b) => b.name === "develop") > -1).toBe(
      true
    );
    expect(store.allBranches.findIndex((b) => b.name === "not") > -1).toBe(
      false
    );
  });

  it("validation should be passed", function () {
    let passedCount = 0;
    const branchNames = [
      "master",
      "main",
      "develop",
      "feature/test/test1",
      "fix/test/test1",
      "hotfix/test/test1",
      "release/test/test1",
    ];
    branchNames.forEach(function (item) {
      const result = validateBranchName(item);
      if (result) passedCount += 1;
    });
    assert.equal(passedCount, branchNames.length, "validation failed");
  });

  it("validation should be rejected", function () {
    let rejectedCount = 0;
    const branchNames = [
      "master/",
      "master_",
      "main/",
      "main_",
      "develop/",
      "develop_",
      "feature_",
      "feature",
      "fix",
      "fix_",
      "hotfix",
      "hotfix_",
      "release",
      "release_",
    ];
    branchNames.forEach(function (item) {
      const result = validateBranchName(item);
      if (!result) rejectedCount += 1;
    });
    assert.equal(rejectedCount, branchNames.length, "validation failed");
  });

  // after create
  // TODO
  // 1. if staged count == 0 switch to new branch
});
