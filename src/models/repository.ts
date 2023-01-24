// invoke
export interface RepoInfo {
  path: string;
  is_bare: boolean;
}

export class Repository {
  // public readonly name: string;
  private readonly path: string;
  public is_bare: boolean;

  public constructor(repoInfo: RepoInfo) {
    // this.name = repoInfo.name;
    this.path = repoInfo.path;
    this.is_bare = repoInfo.is_bare;
  }

  public get repoName(): string {
    const arr = this.path.split("/").reverse().filter(Boolean);
    if (arr.length == 0) return "";
    return arr[0];
  }

  public get repoPath(): string {
    return this.path;
  }
}
