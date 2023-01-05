import { invoke as realInvoke } from "@tauri-apps/api/tauri";

export async function invoke(
  cmd: string,
  args: { [key: string]: any } = {}
): Promise<any> {
  console.log(cmd, args);
  if (cmd === "workdir") {
    return "workdir";
  } else {
    return await realInvoke(cmd, args).catch((e) => {
      throw e;
    });
  }
}
