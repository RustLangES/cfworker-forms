<script lang="ts">
	import type { FormStepQuestionText } from "../../models/Step.d";

  export let step: FormStepQuestionText;
  export let answer: string;

  export let sendAnswer: () => void;
</script>

<input
  bind:value={answer}
  on:input={sendAnswer}
  type="text"
  placeholder="Introduce tu respuesta" />

<style>
  input {
    margin-top: 0.5rem;
    padding: 0.5rem 1rem;
    max-width: 95vw;

    border: double 5px var(--primary);
    border-radius: 50px;
    color: white;
    background: transparent;
    filter: var(--dec-shadow-filter);
  }

  input::placeholder {
    color: var(--secondary);
    opacity: 1;
    font-size: 0.9rem;
  }
</style>
