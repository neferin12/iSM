<template>
  <div class="about">
    <div class="text-center w-100">
      <h1>{{name}}</h1>
    </div>
    <b-table-simple>
      <b-tbody>
        <b-tr>
          <b-td>Version</b-td>
          <b-td>{{ version }}</b-td>
        </b-tr>
        <b-tr>
          <b-td>Tauri Version</b-td>
          <b-td>{{ tauri_version }}</b-td>
        </b-tr>
        <b-tr>
          <b-td>Z3</b-td>
          <b-td>{{ z3_version }}</b-td>
        </b-tr>
      </b-tbody>
    </b-table-simple>
    <p class="text-muted">By Julian Pollinger</p>
  </div>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/core"
import {getName, getVersion, getTauriVersion} from "@tauri-apps/api/app";
import {ref} from "vue";

const name = ref("RiSM GUI");
const version = ref<string>("")
const tauri_version = ref<string>("")
const z3_version = ref<string>("")


// invoke('get_version').then((version_res: unknown) => {
//   version.value = version_res as string;
// })

getVersion().then((res: string) => {
  version.value = res;
})
getName().then((res: string) => {
  name.value = res;
})
getTauriVersion().then((res: string) => {
  tauri_version.value = res;
})

invoke('uses_system_z3').then((res: unknown) => {
  if (res) {
    z3_version.value = "System";
  } else {
    z3_version.value = "Bundled"
  }
})
</script>

<style>
</style>
