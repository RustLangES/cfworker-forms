<script lang="ts">
  import { createEventDispatcher, tick } from 'svelte';
	import { quintOut } from 'svelte/easing';
  import { slide, SlideAxis, SlideDirection } from '$lib/presentation/transitions/slide';

	import type { FormStep } from "../models/Step";

	import FormStepQuestionText from './steps/FormStepQuestionText.svelte';
	import FormStepOptions from "./steps/FormStepOptions.svelte";
	import FormStepOptionsMultiple from "./steps/FormStepOptionsMultiple.svelte";
	import { browser } from '$app/environment';

  export let step: FormStep;
  export let active: boolean;
  export let answer: string = "";
  export let canPrev: boolean;
  export let canNext: boolean;
  export let get_answer: () => string;

	const dispatch = createEventDispatcher<{
    prev: null;
    next: number | null;
    answer: [questionId: number, answer: string];
	}>();

  const [slidein, slideout] = slide({
    duration: 1000,
    easing: quintOut,
    axis: SlideAxis.Y,
    direction: SlideDirection.Backward,
    distance: 50
  });

  let disabled = false;

  $: disabled = step.type !== "text" && answer.trim().length === 0;

  async function updateAnswer(active: boolean) {
    if (active && browser) {
      await tick();
      answer = get_answer();
    }
  }

  $: updateAnswer(active);

  let timer: NodeJS.Timeout;

  function sendAnswer() {
    // Debounce
		clearTimeout(timer);
		timer = setTimeout(() => {
      if (answer) dispatch("answer", [step.id, answer]);
		}, 550);
  }

  let customNext: number | null = null;
</script>

{#if active}
  <section in:slidein out:slideout>

    <h1> {step.title} </h1>

    {#if step.description}
      <p> {step.description} </p>
    {/if}

    {#if step.type === "text"}
      <!-- <FormStepText {step} /> -->
    {:else if step.type === "questionText"}
      <FormStepQuestionText {step} bind:answer {sendAnswer} />
    {:else if step.type === "options" && !step.data.canMultiple}
      <FormStepOptions {step} bind:answer {sendAnswer} bind:customNext />
    {:else if step.type === "options" && step.data.canMultiple}
      <FormStepOptionsMultiple {step} bind:answer {sendAnswer} />
    {:else}
      {@debug step}
    {/if}

    <div>
      <button disabled={!canPrev} on:click={() => canPrev && dispatch("prev")}>
        Anterior
      </button>
      <button {disabled} on:click={() => !disabled && dispatch("next", customNext)}>
        {#if canNext}
          Siguiente
        {:else}
          Finalizar
        {/if}
      </button>
    </div>
  </section>
{/if}

<style>
  section {
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

  @media (max-width: 470px) {
    section {
      right: 1rem;
      width: calc(100vw - 2rem);
    }
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

  button:disabled {
    color: #717171;
    border-color: #717171;
    pointer-events: none;
  }

  button:hover {
    filter: drop-shadow(0px 0px 0px #000)
  }

  @media (max-width: 470px) {
    section {
      align-items: center;
    }

    button {
      min-width: 10rem;
    }
  }
</style>
