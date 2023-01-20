export interface StashEntry {
  /** index - `refs/stash@{0}` */
  readonly index: number;

  readonly message: string;

  /** The SHA of the commit object created as a result of stashing. */
  readonly commit_id: string;
}
