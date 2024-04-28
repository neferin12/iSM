<template>
  <div class="about">
    <b-table-simple>
      <b-tbody>
        <b-tr>
          <b-td>Version</b-td>
          <b-td>{{ version }}</b-td>
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
import {invoke} from '@tauri-apps/api'
import {ref} from "vue";

const version = ref<string>("")
const z3_version = ref<string>("")

invoke('get_version').then((version_res: unknown) => {
  version.value = version_res as string;
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
