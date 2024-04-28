<script setup lang="ts">
import {reactive} from "vue";
import {open} from "@tauri-apps/api/dialog"

const form = reactive({
  votes: '',
  seminars: '',
  modelChecking: false,
  iterations: 1000,
  threads: 4
})

async function selectVotesFile() {
  const selected = await open({
    filters: [{
      name: "CSV",
      extensions: ['csv']
    }]
  })
  if (selected) {
    if (Array.isArray(selected)) {
      form.votes = selected[0];
    } else {
      form.votes = selected;
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
      form.seminars = selected[0];
    } else {
      form.seminars = selected;
    }
  }

}

function run() {

}
</script>

<template>
  <BForm>
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
            v-model="form.votes"
            required
        />
        <template #append>
          <b-button @click="selectVotesFile">Select</b-button>
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
            v-model="form.seminars"
            required
        />
        <template #append>
          <b-button @click="selectSeminarsFile">Select</b-button>
        </template>
      </b-input-group>

    </BFormGroup>

    <BFormGroup
        id="input-group-3"
        class="form-group"
        label="Iterations"
        label-for="input-3"
        description="Number of iterations to run."
    >
      <BFormInput
          id="input-3"
          v-model="form.iterations"
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
          v-model="form.threads"
          type="number"
          required
      />

    </BFormGroup>
    <div class="form-group">
      <BFormCheckbox
          id="checkbox-1"
          v-model="form.modelChecking"
          name="checkbox-1"
      >
        Use model checking (<strong>experimental</strong>)
      </BFormCheckbox>
    </div>

    <b-button type="submit" class="w-100">Calculate</b-button>
  </BForm>
</template>

<style scoped>
.form-group {
  margin-bottom: 20px;
}
</style>