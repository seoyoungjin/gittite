// commit info
export interface CommitSignature {
  name: string;
  email: string;
  time: number;
}

export interface CommitMessage {
  subject: string;
  body: string | null;
}

export interface CommitInfo {
  id: string;
  author: CommitSignature;
  comitter: CommitSignature | null;
  time: number;
  message: CommitMessage;
}

// rev log data
export interface CommitData {
  commit_id: string;
  summary: string;
  time: number;
  author: string;
}
