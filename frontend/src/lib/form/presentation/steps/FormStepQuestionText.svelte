<script lang="ts">
	import type { FormStepQuestionText } from "../../models/Step.d";

  export let step: FormStepQuestionText;
</script>

<h1> {step.data.question} </h1>

{#if step.data.description}
  <p> {step.data.description} </p>
{/if}

<input type="text" placeholder="Introduce tu respuesta" />

<style>
  input {
    margin-top: 0.5rem;
    padding: 0.5rem 1rem;

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
