<script lang="ts">
  import { createEventDispatcher } from 'svelte';
	import { quintOut } from 'svelte/easing';
  import { slide, SlideAxis, SlideDirection } from '$lib/presentation/transitions/slide';

	import type { FormStep } from "../models/Step";

	import FormStepText from "./steps/FormStepText.svelte";
	import FormStepQuestionText from './steps/FormStepQuestionText.svelte';

  export let step: FormStep;
  export let active: boolean;

	const dispatch = createEventDispatcher<{
		next: null; // does not accept a payload
	}>();

  const [slidein, slideout] = slide({
    duration: 1000,
    easing: quintOut,
    axis: SlideAxis.Y,
    direction: SlideDirection.Backward,
    distance: 50
  })
</script>

{#if active}
  <div in:slidein out:slideout>
    {#if step.type === "text"}
      <FormStepText step={step} />
    {:else if step.type === "questionText"}
      <FormStepQuestionText step={step} />
    {:else if step.type === "options"}
      <FormStepText step={step} />
    {:else}
      {@debug step}
    {/if}

    <button on:click={() => dispatch("next")}>
      Next
    </button>
  </div>
{/if}

<style>
  div {
    position: fixed;
    top: 50vh;
    right: 40vw;
    transform: translateY(-50%);

    width: 50vw;
    max-width: 800px;
    height: 50vh;
    max-height: 800px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: start;
    gap: 1rem;
  }

  button {
    margin-top: 0.5rem;
    padding: 0.3rem 1rem;

    color: white;
    background: transparent;
    border: solid 1.5px #FFF;
    font-size: 1rem;

    filter: var(--dec-shadow-filter);
    transition: filter 500ms ease;
  }

  button:hover {
    filter: drop-shadow(0px 0px)
  }
</style>
