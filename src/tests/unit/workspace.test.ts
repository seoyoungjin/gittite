import { describe, it, expect, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";

import { setActivePinia, createPinia } from "pinia";
import { useRepositoryStore } from "@/stores/repository";
import WorkSpace from "@/views/WorkSpace.vue";

import { mockIPC } from "@tauri-apps/api/mocks";
import TestRepoIPC from "../fixtures/test-repo.json";

describe("WorkSpace.vue Test", () => {
  mockIPC((cmd, args) => {
    console.log(">> ", cmd, args);
    if (Object.keys(TestRepoIPC).includes(cmd)) {
      const item = TestRepoIPC[cmd];
      if (typeof item == "function") return item(args);
      else return TestRepoIPC[cmd];
    } else {
      console.log(">>>> ", cmd, args);
      return cmd;
    }
  });

  beforeEach(() => {
    setActivePinia(createPinia());
    store = useRepositoryStore();
  });

  it("renders the page", () => {
    // render the component
    const wrapper = mount(WorkSpace);

    // check sub-components are rendered
    expect(
      wrapper.getComponent({ name: "BranchCreate" }).exists()
    ).toBeTruthy();
    expect(
      wrapper.getComponent({ name: "BranchDelete" }).exists()
    ).toBeTruthy();
    expect(
      wrapper.getComponent({ name: "BranchRename" }).exists()
    ).toBeTruthy();
  });
});
