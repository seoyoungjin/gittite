// vim:ts=2:sts=2:sw=2

import type { BranchCompare, BranchInfo } from "../models/branch";
import type { CommitData, CommitInfo } from "../models/commit";
import type { RepoInfo } from "../models/repository";
import type { Settings } from "../models/settings";
import type { StatusItem } from "../models/status";

import { invoke } from "@tauri-apps/api/tauri";
// import { invoke } from "../tests/helpers/fakeInvoke";

export async function loadSettings(): Promise<Settings> {
  return await invoke("get_settings");
}

export async function saveSettings(value: any) {
  return await invoke("save_settings", { value });
}

export async function get_prop(key: string): Promise<string> {
  return await invoke("get_prop", { key });
}

export async function set_prop(key: string, val: string) {
  return await invoke("set_prop", { key, val });
}

// repository
export async function isGitRepository(path: string): Promise<boolean> {
  return await invoke("is_git_repository", { path });
}

export async function setRepository(path: string): Promise<string> {
  return await invoke("set_repository", { path });
}

export async function getRepositoryInfo(): Promise<RepoInfo> {
  return await invoke("get_repo_info");
}

/// git
export async function init(dirname: string): Promise<string> {
  return await invoke("init", { args: [dirname] });
}

export async function clone(gitUrl: string, localDir: string): Promise<string> {
  return await invoke("clone", { args: [gitUrl, localDir] });
}

// history/diff
export async function getCommits(
  revisionRange?: string,
  limit?: number,
  skip?: number,
  additionalArgs?: string[]
): Promise<CommitData[]> {
  const args = [];

  // TODO
  // if (revisionRange != undefined) {
  //   args.push(revisionRange);
  // }
  if (limit != undefined) {
    args.push(`--max-count=${limit}`);
  }
  if (skip != undefined) {
    args.push(`--skip=${skip}`);
  }
  if (additionalArgs != undefined) {
    args.push(...additionalArgs);
  }

  return await invoke("get_commits", { args: args });
}

export async function getDiff(path: string, stage: boolean): Promise<string> {
  return await invoke("get_diff", { path: path, stage: stage });
}

export async function getDiffCommit(
  commitId: string,
  path: string | null
): Promise<string> {
  return invoke("get_diff_commit", { commitId: commitId, path: path });
}

// stage
export async function add(name: string): Promise<boolean> {
  return invoke("add", { args: name });
}

export async function remove(name: string): Promise<boolean> {
  return invoke("remove", { args: name });
}

export async function resetStage(name: string): Promise<boolean> {
  return invoke("reset_stage", { args: name });
}

export async function getStatus(args: string): Promise<StatusItem[]> {
  return await invoke("get_status", { statusType: args });
}

// commit
export async function commit(message: string): Promise<string> {
  return await invoke("commit", { args: message });
}

export async function commitAmend(message: string): Promise<string> {
  return await invoke("amend", { args: message });
}

export async function commitInfo(commitId: string): Promise<CommitInfo> {
  return await invoke("commit_info", { args: commitId });
}

export async function commitFiles(commitId: string): Promise<StatusItem[]> {
  return await invoke("commit_files", { args: commitId });
}

// branch
export async function getBranchName(): Promise<string> {
  return await invoke("get_branch_name");
}

export async function createBranch(name: string): Promise<string> {
  return await invoke("create_branch", { args: name });
}

export async function deleteBranch(name: string) {
  return await invoke("delete_branch", { args: name });
}

export async function renameBranch(branch: string, name: string) {
  return await invoke("rename_branch", { branch: branch, name: name });
}

export async function branchesInfo(local: boolean): Promise<BranchInfo[]> {
  return await invoke("get_branches_info", { local: local });
}

export async function branchRemote(branch: string): Promise<string | null> {
  return await invoke("get_branch_remote", { branch: branch });
}

export async function branchCompareUpstream(
  branch: string
): Promise<BranchCompare> {
  return await invoke("branch_compare_upstream", { branch: branch });
}

export async function checkoutBranch(branch: string) {
  return await invoke("checkout_branch", { branchRef: branch });
}

export async function checkoutRemoteBranch(branch: string) {
  return await invoke("checkout_remote_branch", { branchRef: branch });
}

// tag
export async function tagAdd(
  tagname: string,
  object: string | null,
  message: string | null,
  force: boolean
) {
  const arr: string[] = ["add", tagname];
  if (object) arr[arr.length] = object;
  if (force) arr[arr.length] = "-f";
  if (message) {
    arr[arr.length] = "-m";
    arr[arr.length] = message;
  }
  return await invoke("tag", { args: arr });
}

export async function tagList(pattern: string | null) {
  if (pattern == null) pattern = "";
  return await invoke("tag", { args: ["list", pattern] });
}

export async function tagDelete(tagname: string) {
  return await invoke("tag", { args: ["delete", tagname] });
}

// stash
export async function stashSave(
  message: string | null,
  includeUntracked: boolean,
  keepIndex: boolean
) {
  const arr = ["save", message];
  if (includeUntracked) arr[arr.length] = "-u";
  if (keepIndex) arr[arr.length] = "-k";
  return await invoke("stash", { args: arr });
}

export async function stashList() {
  return await invoke("stash", { args: ["list"] });
}

export async function stashApply(stashid: string) {
  return await invoke("stash", { args: ["apply", stashid] });
}

export async function stashPop(stashid: string) {
  return await invoke("stash", { args: ["pop", stashid] });
}

export async function stashDrop(stashid: string) {
  return await invoke("stash", { args: ["drop", stashid] });
}

// remote
export async function fetch() {}

export async function push() {}

// blame
export async function blameFile(path: string, commitId: string | null) {
  return await invoke("blame", { path: path, commitId: commitId });
}

// config/ignore
export async function addToIgnore(path: string) {
  return await invoke("add_to_ignore", { path });
}

// test progress
export async function testProgress() {
  return await invoke("test_progress");
}
