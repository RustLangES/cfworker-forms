<script lang="ts">

  import type { Form } from "$lib/form/models/Form.d"
	import FormStep from "$lib/form/presentation/FormStep.svelte";

  export let data: Form;

  let actual_step: number = 0;

</script>

<h1> {data.title} </h1>


{#each data.steps as step, step_idx}
  <FormStep 
    {step}
    active={step_idx == actual_step}
    on:next={() => actual_step++} />
{/each}

<footer> RustLangES Forms </footer>

<style>
  :global(body) {
    height: 100vh;
  }

  h1 {
    font-size: 1.5rem;
    position: fixed;
    top: 1rem;
    left: 50vw;
    transform: translateX(-50%);

    font-family: var(--fonts-heading);
    color: var(--title);
  }

  footer {
    position: fixed;
    bottom: 1rem;
    left: 50vw;
    transform: translateX(-50%);

    font-size: 0.8rem;
    font-family: var(--fonts-heading);
    color: var(--title);
  }

</style>
