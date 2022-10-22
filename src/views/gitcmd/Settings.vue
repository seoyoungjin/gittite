<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { configDir } from '@tauri-apps/api/path';
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';

let jsonData = await invoke('get_settings');
/* TODO
invoke('get_settings')
  .then(function(data) {
    jsonData = data;
  })
  .catch(function(error) {
    jsonData = { error: error };
  });
  */

var jsonData2 = await readTextFile('gittite/settings.json', { dir: BaseDirectory.Config});
/*
readTextFile('gittite/settings.json', { dir: BaseDirectory.Config})
  .then(function(data) {
    alert(data);
    jsonData2 = JSON.parse(data);
  }).catch(function(err) {
    jsonData2 = { "error": err };
  });
  */

export default {
  data() {
    return {
      jsonData: jsonData,
      jsonData2: jsonData2,
    }
  }
}
</script>

<template>

<h5> Settings </h5>

We can read settings file with two methods.
<br />
<br />
1. rust with tauri command
<br />
2. javascript using "@tauri-apps/api/fs"
<br />
<br />

<h6> 1. via tauri::command </h6>
<json-viewer :value="jsonData"></json-viewer>
<br />

<h6> 2. with Javascript </h6>
<json-viewer :value="jsonData2"></json-viewer>

</template>