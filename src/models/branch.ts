export interface BranchCompare {
  ahead: number;
  behind: number;
}

// branch info
export interface LocalBranch {
  is_head: boolean;
  has_upstream: boolean;
  remote: string | null;
}

export interface RemoteBranch {
  has_tracking: boolean;
}

export interface BranchDetails {
  Local?: LocalBranch;
  Remote?: RemoteBranch;
}

export interface BranchInfo {
  name: string;
  reference: string;
  top_commit_message: string;
  top_commit: string;
  details: BranchDetails;
}
