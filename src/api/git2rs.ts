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

/// git
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
  try {
    return await invoke("get_status", { statusType: args });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function blameFile(path: string, commitId: string | null) {
  try {
    return await invoke("blame", { path: path, commitId: commitId });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

// commit related
export async function commit(message: string | null) {
  return await invoke("commit", { args: message });
}

export async function commitAmend(message: string | null) {
  return await invoke("amend", { args: message });
}

export async function commitInfo(commitId: string) {
  try {
    return await invoke("commit_info", { args: commitId });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function commitFiles(commitId: string) {
  try {
    return await invoke("commit_files", { args: commitId });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

// branch
export async function createBranch(name: string) {
  try {
    return await invoke("create_branch", { args: name });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function deleteBranch(name: string) {
  try {
    return await invoke("delete_branch", { args: name });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function renameBranch(branch: string, name: string) {
  try {
    return await invoke("rename_branch", { branch: branch, name: name });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function branchesInfo(local: bool) {
  try {
    return await invoke("get_branches_info", { local: local });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function branchRemote(branch: string) {
  try {
    return await invoke("get_branch_remote", { branch: branch });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function branchCompareUpstream(branch: string) {
  try {
    return await invoke("branch_compare_upstream", { branch: branch });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function checkoutBranch(branch: string) {
  try {
    return await invoke("checkout_branch", { branchRef: branch });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function checkoutRemoteBranch(branch: string) {
  try {
    return await invoke("checkout_remote_branch", { branchRef: branch });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

// tag
export async function tagAdd(
  tagname: string,
  object: string | null,
  message: string | null,
  force: boolean
) {
  try {
    const arr: string[] = ["add", tagname];
    if (object) arr[arr.length] = object;
    if (force) arr[arr.length] = "-f";
    if (message) {
      arr[arr.length] = "-m";
      arr[arr.length] = message;
    }
    return await invoke("tag", { args: arr });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function tagList(pattern: string | null) {
  try {
    if (pattern == null) pattern = "";
    return await invoke("tag", { args: ["list", pattern] });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function tagDelete(tagname: string) {
  try {
    return await invoke("tag", { args: ["delete", tagname] });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

// stash
export async function stashSave(
  message: string | null,
  includeUntracked: boolean,
  keepIndex: boolean
) {
  try {
    const arr = ["save", message];
    if (includeUntracked) arr[arr.length] = "-u";
    if (keepIndex) arr[arr.length] = "-k";
    return await invoke("stash", { args: arr });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashList() {
  try {
    return await invoke("stash", { args: ["list"] });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashApply(stashid: string) {
  try {
    return await invoke("stash", { args: ["apply", stashid] });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashPop(stashid: string) {
  try {
    return await invoke("stash", { args: ["pop", stashid] });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashDrop(stashid: string) {
  try {
    return await invoke("stash", { args: ["drop", stashid] });
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}
