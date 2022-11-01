// vim:ts=2:sts=2:sw=2

import { invoke } from "@tauri-apps/api/tauri";

export async function runCmd<T = any>(
  cmd: string,
  options: { [key: string]: any } = {}
) {
  return (await invoke(cmd, options).catch((e) => {
    alert(e);
    throw e;
  })) as T;
}

export async function loadSettings() {
  return await runCmd("get_settings").then((res) => {
    return res;
  });
}

export async function add(name): Promise<bool> {
  return invoke("add", { args: name });
}

export async function remove(name): Promise<bool> {
  return invoke("remove", { args: name });
}

export async function resetStage(name): Promise<bool> {
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
