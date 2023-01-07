import { invoke as realInvoke } from "@tauri-apps/api/tauri";

// import { TestRepoIPC } from "../fixtures/test-repo";
import { TestRepoIPC } from "../fixtures/repo-with-many-commits.ts";

export async function invoke(
  cmd: string,
  args: { [key: string]: any } = {}
): Promise<any> {
  console.log(">> ", cmd, args);
  if (Object.keys(TestRepoIPC).includes(cmd)) {
    const item = TestRepoIPC[cmd];
    if (typeof item == "function") {
      return item(args);
    } else {
      return TestRepoIPC[cmd];
    }
  } else {
    return await realInvoke(cmd, args).catch((e) => {
      console.log("invoke error: ", e);
      throw e;
    });
  }
}
