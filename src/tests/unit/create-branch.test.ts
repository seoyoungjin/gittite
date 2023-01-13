import { describe, it, expect, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useRepositoryStore } from "@/stores/repository";

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

  it("test create branch", async () => {
    await store.setRepository(".");

    expect(store.currentBranch).toBe("master");

    // check exist branch
    expect(store.branchExist("develop")).toBe(true);
    expect(store.branchExist("not_exist")).toBe(false);

    // TODO
    // button enable?
    // error message

    // switch with changes
  });
});
