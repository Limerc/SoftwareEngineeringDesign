import type { Problem } from "@/apis/problems";
import { defineStore } from "pinia";
import { ref } from "vue";

export const useProblemStore = defineStore('problem', () => {
  const problem = ref<Problem>();

  function setProblem(p: Problem) {
    problem.value = p;
  }

  function clearProblem() {
    problem.value = undefined;
  }

  function getProblem() {
    return problem.value;
  }

  return {
    problem,
    setProblem,
    clearProblem,
    getProblem
  }
})
