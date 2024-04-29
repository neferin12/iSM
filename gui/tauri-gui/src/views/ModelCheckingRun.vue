<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {onMounted, reactive} from "vue";
import {listen} from "@tauri-apps/api/event";
import {storeToRefs} from "pinia";
import {useDataStore} from "@/stores/data";
import {useRouter} from "vue-router";

const {votes, seminars, result} = storeToRefs(useDataStore());

const router = useRouter();

async function run() {
  const resString: string = await invoke("run_model_checking", {
    votes: votes.value,
    seminars: seminars.value
  });
  result.value = JSON.parse(resString);
  router.push({name: "results"});
}

onMounted(() => {
  run()
});
</script>

<template>
  <BProgress variant="primary" :max="100" :value="100" class="mb-4" striped animated/>

  <BAlert variant="warning" :model-value="true">
    <h3>Warning</h3>
    This feature is highly experimental. If the problem is sufficiently large, the calculation may not terminate.
  </BAlert>
</template>

<style scoped>
#mc-content {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>