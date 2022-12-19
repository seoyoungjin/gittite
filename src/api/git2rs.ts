// vim:ts=2:sts=2:sw=2

import { invoke } from "@tauri-apps/api/tauri";

export async function runCmd<T = any>(
  cmd: string,
  options: { [key: string]: any } = {}
) {
  return (await invoke(cmd, options).catch((e) => {
    throw e;
  })) as T;
}

export async function loadSettings() {
  return await runCmd("get_settings").then((res) => {
    return res;
  });
}

export async function saveSettings(value: any) {
  return await runCmd("save_settings", { value }).then((res) => {
    return res;
  });
}

export async function get_prop(key: string): Promise<string> {
  return await invoke("get_prop", { key });
}

export async function set_prop(key: string, val: string) {
  return await invoke("set_prop", { key, val });
}

// repository
export async function isGitRepository(path: string) {
  return await invoke("is_git_repository", { path });
}

export async function setRepository(path: string): Promise<string> {
  return await invoke("set_repository", { path });
}

/// git
export async function init(dirname: string) {
  return await invoke("init", { args: [dirname] });
}

export async function clone(gitUrl: string, localDir: string) {
  return await runCmd("clone", { args: [gitUrl, localDir] }).then((res) => {
    return res;
  });
}

export async function getCommits() {
  return await runCmd("get_commits", { args: [] }).then((res) => {
    return res;
  });
}

export async function getDiff(path: string, stage: boolean) {
  return await runCmd("get_diff", { path: path, stage: stage }).then((res) => {
    return res;
  });
}

export async function getDiffCommit(commitId: string, path: string) {
  return invoke("get_diff_commit", { commitId: commitId, path: path });
}

export async function add(name: string): Promise<boolean> {
  return invoke("add", { args: name });
}

export async function remove(name: string): Promise<boolean> {
  return invoke("remove", { args: name });
}

export async function resetStage(name: string): Promise<boolean> {
  return invoke("reset_stage", { args: name });
}

export async function getStatus(args: string) {
  return await invoke("get_status", { statusType: args });
}

export async function blameFile(path: string, commitId: string | null) {
  return await invoke("blame", { path: path, commitId: commitId });
}

// commit related
export async function commit(message: string | null) {
  return await invoke("commit", { args: message });
}

export async function commitAmend(message: string | null) {
  return await invoke("amend", { args: message });
}

export async function commitInfo(commitId: string) {
  return await invoke("commit_info", { args: commitId });
}

export async function commitFiles(commitId: string) {
  return await invoke("commit_files", { args: commitId });
}

// branch
export async function getBranchName() {
  return await invoke("get_branch_name");
}

export async function createBranch(name: string) {
  return await invoke("create_branch", { args: name });
}

export async function deleteBranch(name: string) {
  return await invoke("delete_branch", { args: name });
}

export async function renameBranch(branch: string, name: string) {
  return await invoke("rename_branch", { branch: branch, name: name });
}

export async function branchesInfo(local: boolean) {
  return await invoke("get_branches_info", { local: local });
}

export async function branchRemote(branch: string) {
  return await invoke("get_branch_remote", { branch: branch });
}

export async function branchCompareUpstream(branch: string) {
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
