<script lang="ts">
  import { FallingConfetti } from 'svelte-canvas-confetti';

	import type { Answer } from "$lib/form/models/Answer";
	import type { Form } from "$lib/form/models/Form";

  import FormStep from "$lib/form/presentation/FormStep.svelte";
	import TopTitle from "$lib/presentation/TopTitle.svelte";
	import ButtonLink from '$lib/presentation/ButtonLink.svelte';

  export let form: Form;
  export let user: string | null;
  export let answers: Answer[];
  export let API_HOST: string;

  const clamp = (value: number, min: number, max: number) => Math.max(Math.min(value, max), min);

  const actual_step_idx = answers.findLastIndex(answer => answer.data.trim().length);
  let actual_step: number = clamp(actual_step_idx, 0, form.questions.length - 1);

  $: ended = actual_step >= form.questions.length;

  function get_answer(questionId: number) {
    let old_answer = answers.findIndex(answer => answer.question_id === questionId);

    if (old_answer !== -1 && answers[old_answer].data) {
      return answers[old_answer].data.trim();
    }

    return "";
  }

  async function answer(ev: CustomEvent<[questionId: number, answer: string]>) {
    const questionId = ev.detail[0];
    const answer = ev.detail[1];

    const res = await fetch(`${API_HOST}/api/form/${form.id}/question/${questionId}/answer`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${user}`
      },
      body: JSON.stringify({ data: answer })
    });

    if (!res.ok) {
      console.error(res);
      return false;
    }

    const answerIdx = answers.findIndex(answer => answer.question_id === questionId);

    if (answerIdx === -1) {
      answers.push({
        question_id: questionId,
        data: answer
      });
    } else {
      answers[answerIdx].data = answer;
    }

    return true; 
  }

  async function next() {
    actual_step++;

    if (actual_step >= form.questions.length) {
      // Delete session when ends form
      await fetch(`${API_HOST}/api/form/${form.id}/session`, {
        method: "DELETE",
        headers: {
          "Authorization": `Bearer ${user}`
        }
      });
    }
  }

  $: step_bar_progress = (actual_step + 1) / form.questions.length * 100;
</script>

<div
  class="step-bar"
  style:--step={`${step_bar_progress.toFixed(2)}%`} />

<TopTitle> {form.title} </TopTitle>

{#each form.questions as step, step_idx}
<FormStep 
  {step}
  active={step_idx === actual_step}
  get_answer={() => get_answer(step.id)}

  canPrev={step_idx !== 0}
  canNext={step_idx !== form.questions.length - 1}

  on:prev={() => actual_step--}
  on:answer={answer}
  on:next={next} />
{:else}
  Empty Form :)
{/each}

{#if ended}
  <FallingConfetti />

  <h2>Finalizaste el Formulario</h2>

  <ButtonLink href="/"> Ir al inicio </ButtonLink>
{/if}

<style>
  h2 {
    font-size: 2rem;
    margin-bottom: 2rem;

    text-transform: capitalize;
  }

  .step-bar {
    --step: 0%;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 10px;
    background: #000;
  }

  .step-bar::before {
    content: "";
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;

    width: var(--step);
    background: var(--primary);

    transition: width 500ms ease;
  }
</style>
