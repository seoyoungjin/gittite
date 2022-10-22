<template>
  <div class="about">
    <h2>Log</h2>

    <json-viewer :value="jsonData"></json-viewer>
  </div>
</template>

<script>

  var json = {
      "name": "Molecule Man",
      "age": 29,
      "secretIdentity": "Dan Jukes",
      "powers": [
        "Radiation resistance",
        "Turning tiny",
        "Radiation blast"
      ]
    };

  export default {
    data() {
      return {
        jsonData: json
      }
    }
  }
</script>
