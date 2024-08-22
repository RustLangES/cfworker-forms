<script lang="ts">
  import { fly } from "svelte/transition"

	import Layout from "$lib/presentation/Layout.svelte";
	import type { Form } from "$lib/form/models/Form.d";

  const defaultForm = {
    id: 0,
    title: "Rust Lang Es",
    edition: "",
    description: ""
  };

  type FormDetails = typeof defaultForm;

  export let data: { forms: Form[] };


  let actualFormDetails = defaultForm;
  let actualFormSelected = 0;
  let isTouch = false;

  const clickOnForm = (ev: Event) => {
    if (!isTouch) {
      actualFormDetails.id = 0;
      return;
    }

    if (actualFormDetails.id === actualFormSelected) {
      actualFormDetails.id = 0;
    } else {
      ev.preventDefault();
      actualFormDetails = data.forms[actualFormSelected - 1];
    }
  };

  const mouseEnter = (newForm: FormDetails) => () => {
    if (!isTouch) {
      actualFormDetails = newForm;
    }
  };
  const mouseLeave = () => {
    actualFormSelected = 0;
    actualFormDetails = defaultForm;
  };

  const touchStart = (form: FormDetails) => () => {
    actualFormSelected = form.id;
    isTouch = true;
  };
</script>

<Layout container>
  <header transition:fly={{ y: 50 }}>
    <h1>{actualFormDetails.title}</h1>
    <p class="edition">{actualFormDetails.edition ?? ""}</p>
    <p class="description">{actualFormDetails.description ?? ""}</p>
  </header>

  <ul on:mouseleave={mouseLeave}>
    {#each data.forms as form, idx}
      <li class:dimmed={actualFormSelected !== 0 && actualFormSelected !== idx + 1}>
        <a
          href={`/form/${form.id}`}

          on:mouseenter={mouseEnter(form)}

          on:touchstart={touchStart(form)}

          on:click={clickOnForm}
        >
          <span>{form.title}</span>
          <span>&gt;</span>
        </a>
      </li>
    {/each}
  </ul>
</Layout>

<style>
  :global(body) {
    display: grid;
    place-items: center;
  }

  header {
    margin-top: max(2rem, 2vh);
    padding-top: max(2rem, 2vh);

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  h1 {
    font-size: 3rem;
    letter-spacing: 0.05ch;

    font-family: var(--fonts-heading);
    font-weight: 400;
    color: var(--title);
  }

  p {
    min-height: 2rem;
  }

  .edition {
    font-size: 1.25rem;
    font-family: var(--fonts-heading);
  }

  .description {
    margin-top: 2rem;
  }

  ul {
    height: min-content;
    padding: 2rem;

    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  li {
    border: solid 1px #FFF;
    color: white;

    filter: var(--dec-shadow-filter);
    transition: filter 500ms ease;
  }

  li:hover {
    filter: drop-shadow(0px 0px 0px #000)
  }

  li.dimmed {
    opacity: 0.8;
    scale: 0.95;
  }

  a {
    width: 100%;
    height: 100%;
    padding: 1rem 2rem;

    color: currentColor;
    text-decoration: none;

    display: flex;
    justify-content: space-between;
  }
</style>
