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

export interface DiffOptions {
  ignore_whitespace: boolean;
  ontext: number;
  interhunk_lines: number;
}

// settings
export interface Profile {
  name: string;
  email: string;
  image_url: string;
}

export interface Settings {
  profile: Profile;
  all_repository: string[];
}

export interface StatusItem {
  path: string;
  stage?: string;
  wtree?: string;
}
