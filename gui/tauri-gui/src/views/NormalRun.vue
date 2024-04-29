<script setup lang="ts">
import {onMounted, reactive} from "vue";
import {useDataStore} from "@/stores/data";
import {storeToRefs} from "pinia";
import {invoke} from "@tauri-apps/api";
import {listen} from "@tauri-apps/api/event";
import {useRouter} from "vue-router";

const {votes, seminars, iterations, threads, result} = storeToRefs(useDataStore());

const progress = reactive(new Map<string, { progress: number, total: number }>());
const router = useRouter();

async function run() {
  const resString: string = await invoke("run_normal", {
    votes: votes.value,
    seminars: seminars.value,
    iterations: Number.parseInt(iterations.value.toString()),
    threads: Number.parseInt(threads.value.toString())
  });
  result.value = JSON.parse(resString);
  router.push({name: "results"});
}

onMounted(() => {
  listen('progress', (event: any) => {
    progress.set(event.payload.thread_id, {progress: event.payload.progress, total: event.payload.total});
  })
  run()
});
</script>

<template>
  <p><strong>Running...</strong></p>
  <div v-for="(value, key) in progress" :key="key" class="mb-3">
    <p class="mb-0">Thread {{ key + 1 }}</p>
    <b-progress :value="value[1].progress" :max="value[1].total">
    </b-progress>
  </div>
</template>

<style scoped>

</style>