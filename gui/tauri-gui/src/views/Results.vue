<script setup lang="ts">
import {useDataStore} from "@/stores/data";
import {storeToRefs} from "pinia";
import {onMounted} from "vue";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';

const {result} = storeToRefs(useDataStore());

const tableData = result.value?.assignments?.map((a: Assignment) => {
  return {
    name: a.student.name,
    points: a.points,
    w_seminar: a.w_seminar?.name,
    p_seminar: a.p_seminar?.name
  }
});

const fields =[
  { key: 'name', label: 'Name', sortable: true },
  { key: 'points', label: 'Points', sortable: true },
  { key: 'w_seminar', label: 'W Seminar', sortable: true },
  { key: 'p_seminar', label: 'P Seminar', sortable: true }

]

onMounted(async () => {
// Do you have permission to send a notification?
  let permissionGranted = await isPermissionGranted();

// Once permission has been granted we can send the notification
  if (permissionGranted) {
    sendNotification({ title: 'iSM run done', body: 'iSM has finished the calculation!' });
  }
});
</script>

<template>
  <h4>Total Points: {{ result?.points }}</h4>
  <h4>Assignments:</h4>
  <BTable :items="tableData" :fields="fields"></BTable>
</template>

<style scoped>

</style>