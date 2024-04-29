import {defineStore} from "pinia";
import {type Ref, ref} from "vue";

export const useDataStore = defineStore('data', () => {
    const votes = ref('');
    const seminars = ref('');
    const modelChecking = ref(false);
    const iterations = ref(1000000);
    const threads = ref(4);

    const result: Ref<Result | null> = ref(null);

    return {votes, seminars, modelChecking, iterations, threads, result}
})