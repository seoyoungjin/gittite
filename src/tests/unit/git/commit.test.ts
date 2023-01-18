import { setupFixtureRepository } from "../../helpers/repositories";

describe("git/commit", () => {
  beforeEach(async () => {
    const testRepoPath = await setupFixtureRepository("repo-with-changes");
  });

  it("test", () => {
    assert.equal(3, 3);
  });
});

// after commit
// TODO
// 1. staged count == 0
// 2. last commit id == first item of history
