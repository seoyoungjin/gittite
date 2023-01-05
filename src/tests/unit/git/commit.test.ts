import * as path from 'path';
import * as FSE from 'fs-extra';

import {
  setupFixtureRepository,
} from '../../helpers/repositories';

describe('git/commit', () => {

  beforeEach(async () => {
    const testRepoPath = await setupFixtureRepository('repo-with-changes')
  })

  it('test', () => {
    assert.equal(3, 3)
  })

})
