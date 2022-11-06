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

export async function add(name): Promise<boolean> {
  return invoke("add", { args: name });
}

export async function remove(name): Promise<boolean> {
  return invoke("remove", { args: name });
}

export async function resetStage(name): Promise<boolean> {
  return invoke("reset_stage", { args: name });
}

export async function getStatus(args: string) {
  try {
    return await invoke("get_status", { statusType: args });
  } catch (e) {
    if (typeof e == "string") {
      return { error: e };
    } else {
      return { error: JSON.stringify(e) };
    }
  }
}

// tag
export async function tagAdd(
    tagname: string,
    object: string | null,
    message: string | null,
    force: boolean)
{
  try {
    var arr: (string)[] = ["add", tagname];
    if (object)
        arr[arr.length] = object;
    if (force)
        arr[arr.length] = "-f";
    if (message) {
        arr[arr.length] = "-m";
        arr[arr.length] = message;
    }
    return await invoke("tag", {args: arr});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function tagList(pattern: string | null) {
  try {
    if (pattern == null)
        pattern = "";
    return await invoke("tag", {args: ["list", pattern]});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function tagDelete(tagname: string) {
  try {
    return await invoke("tag", {args: ["delete", tagname]});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

// stash
export async function stashSave(
    message: string | null,
    includeUntracked: boolean,
    keepIndex: boolean)
{
  try {
    var arr = ["save", message];
    if (includeUntracked)
        arr[arr.length] = "-u";
    if (keepIndex)
        arr[arr.length] = "-k";
    return await invoke("stash", {args: arr});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashList() {
  try {
    return await invoke("stash", {args: ["list"]});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashApply(stashid: string) {
  try {
    return await invoke("stash", {args: ["apply", stashid]});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashPop(stashid: string) {
  try {
    return await invoke("stash", {args: ["pop", stashid]});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}

export async function stashDrop(stashid: string) {
  try {
    return await invoke("stash", {args: ["drop", stashid]});
  } catch (e) {
    return { error: JSON.stringify(e) };
  }
}
