// fake log data for develop
//
// - load this in fakeInvoek.ts
// - replace invoke() with fakeInvoke() in git2rs.ts
// - yarn tauri dev

// import TestRepoObject from "./test-repo.json";

function get_commits(): [] {
  const now = Date.now();
  const log_list = [];

  for (let i = 0; i < 1000; i++) {
    log_list.push({
      author: "Author",
      commit_id: "" + i,
      summary: "" + i,
      time: Math.floor(now / 1000),
      // time: 1672988315,
    });
  }
  return log_list;
}

class TestIPC {
  constructor(data: any) {
    // Object.keys(data).forEach((k) => (this[k] = data[k]));
    this.get_commits = get_commits();
  }
}

// export const TestRepoIPC = new TestIPC(TestRepoObject);
export const TestRepoIPC = new TestIPC(null);
