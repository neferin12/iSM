<script setup lang="ts">
import {onMounted, reactive} from "vue";
import {open} from "@tauri-apps/plugin-dialog"
import {useRouter} from "vue-router";
import {storeToRefs} from "pinia";
import {useDataStore} from "@/stores/data";

const {votes, seminars, modelChecking, iterations, threads, result} = storeToRefs(useDataStore());

async function selectVotesFile() {
  const selected = await open({
    filters: [{
      name: "CSV",
      extensions: ['csv']
    }]
  })
  if (selected) {
    if (Array.isArray(selected)) {
      votes.value = selected[0];
    } else {
      votes.value = selected;
    }
  }

}

async function selectSeminarsFile() {
  const selected = await open({
    filters: [{
      name: "CSV",
      extensions: ['csv']
    }]
  })
  if (selected) {
    if (Array.isArray(selected)) {
      seminars.value = selected[0];
    } else {
      seminars.value = selected;
    }
  }

}

const router = useRouter();

function run(evt: Event) {
  evt.preventDefault();
  if (modelChecking.value) {
    router.push({name: "run-model-checking"})
  } else {
    router.push({name: "run-normal"});
  }
}

onMounted(() => {
  result.value = null;
});

</script>

<template>
  <BForm @submit="run">
    <BFormGroup
        id="input-group-1"
        class="form-group"
        label="Votes file"
        label-for="input-1"
        description="CSV file containing the students' votes."
    >
      <b-input-group>
        <BFormInput
            id="input-1"
            v-model="votes"
            required
        />
        <template #append>
          <b-button @click="selectVotesFile" variant="outline-primary">Select</b-button>
        </template>
      </b-input-group>

    </BFormGroup>

    <BFormGroup
        id="input-group-2"
        class="form-group"
        label="Seminars file"
        label-for="input-2"
        description="CSV file containing the seminars."
    >
      <b-input-group>
        <BFormInput
            id="input-2"
            v-model="seminars"
            required
        />
        <template #append>
          <b-button @click="selectSeminarsFile" variant="outline-primary">Select</b-button>
        </template>
      </b-input-group>

    </BFormGroup>

    <BCollapse :visible="!modelChecking">
      <BFormGroup
        id="input-group-3"
        class="form-group"
        label="Iterations"
        label-for="input-3"
        description="Number of iterations to run."
    >
      <BFormInput
          id="input-3"
          v-model="iterations"
          type="number"
          required
      />

    </BFormGroup>

    <BFormGroup
        id="input-group-4"
        class="form-group"
        label="Threads"
        label-for="input-4"
        description="Number of threads to use."
    >
      <BFormInput
          id="input-3"
          v-model="threads"
          type="number"
          required
      />

    </BFormGroup>
    </BCollapse>
    <div class="form-group">
      <BFormCheckbox
          id="checkbox-1"
          v-model="modelChecking"
          name="checkbox-1"
      >
        Use model checking (<strong>experimental</strong>)
      </BFormCheckbox>
    </div>

    <b-button type="submit" class="w-100" variant="primary">Calculate</b-button>
  </BForm>
</template>

<style scoped>
.form-group {
  margin-bottom: 20px;
}
</style>