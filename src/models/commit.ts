import type { Tag } from "@/models/tag";

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

export class Commit {
  public readonly commit_id: string;
  public readonly summary: string;
  public readonly time: number;
  public readonly author: string;
  public tags: Tag[] | undefined;

  public constructor(data: CommitData) {
    this.commit_id = data.commit_id;
    this.summary = data.summary;
    this.time = data.time;
    this.author = data.author;

    this.tags = [];
  }
}
