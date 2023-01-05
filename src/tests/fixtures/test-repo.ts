// fixture with object
import TestRepoObject from "../fixtures/test-repo.json";

function get_status(args) {
  console.log("GET_TATUS", args);
  if (args.status_type == "workdir") {
    console.log("GET_TATUS", args);
  }
}

class TestIPC {
  constructor(data: any) {
    Object.keys(data).forEach((k) => (this[k] = data[k]));

    this.get_status = get_status;
  }
}

export const TestRepoIPC = new TestIPC(TestRepoObject);
