// fixture with object
import TestRepoObject from "../fixtures/test-repo.json";

function get_status(args) {
  if (args.status_type == "stage") {
    return [];
  }
  return [];
}

class TestIPC {
  constructor(data: any) {
    Object.keys(data).forEach((k) => (this[k] = data[k]));

    this.get_status = get_status;
  }
}

export const TestRepoIPC = new TestIPC(TestRepoObject);
