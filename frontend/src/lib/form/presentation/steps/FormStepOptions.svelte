<script lang="ts">
	import type { FormStepOptions } from "../../models/Step.d";

  export let step: FormStepOptions;
  export let answer: string;

  export let sendAnswer: () => void;
</script>

<ul>
  {#each step.data.options as option, idx}
    <li>
      <input 
        id={`q-${step.id}-op-${idx}`}
        name="option"
        type="radio"
        bind:group={answer}
        value={option}
        on:change={sendAnswer} />

      <label for={`q-${step.id}-op-${idx}`}>
        <span />
        {option}
      </label>
    </li>
  {/each}
</ul>

<style>
  ul {
    display: flex;
    flex-direction: column;
  }

  li {
    display: flex;
  }

  input {
    position: absolute;
    opacity: 0;
  }

  label {
    box-sizing: content-box;

    margin-top: calc(0.5rem + 3px);
    margin-bottom: 3px;
    margin-left: 3px;
    padding: 0.5rem 1.25rem;
    width: 20rem;
    max-width: calc(90vw - 2.5rem);

    border: solid 2px var(--primary);
    border-radius: 50px;
    color: white;
    background: transparent;
    filter: var(--dec-shadow-filter);

    display: flex;
    align-items: center;
    gap: 1rem;

    transition: border-color 300ms ease, filter 200ms ease;
  }

  label:hover {
    filter: drop-shadow(0px 0px 0px #000)
  }

  input:checked + label {
    margin-top: 0.5rem;
    margin-bottom: 0;
    margin-left: 0px;

    border-width: 5px;
  }

  input:focus + label {
    border-color: var(--secondary);
  }

  label span {
    display: block;
    width: 1rem;
    height: 1rem;

    border: solid 1px var(--primary);
    border-radius: 100%;

    position: relative;
    transition: border-color 300ms ease;
  }

  input:checked + label span {
    border-color: transparent;
  }

  input:focus + label span {
    border-color: var(--secondary);
  }

  label span::before {
    content: "";
    position: absolute;
    inset: 0;

    background: var(--primary);
    border-radius: 100%;

    transform: scale(0);
    transition: transform 300ms ease, background 300ms ease;
  }

  input:checked + label span::before {
    transform: scale(1);
  }

  input:focus + label span::before {
    background: var(--secondary);
  }
</style>
