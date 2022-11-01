<script>
export default {
  data() {
    return {
      message: "Hello World!",
      appName: "Unknown",
      appVersion: "0.0.0!",
      tauriVersion: "0.0.0!",
    };
  },
};
</script>

<template>
  <h4>Test</h4>
  {{ message }}

  <br />

  <pre>
  name : {{ appName }}
  version : {{ appVersion }}
  tauriVersion : {{ tauriVersion }}
  </pre>
</template>
