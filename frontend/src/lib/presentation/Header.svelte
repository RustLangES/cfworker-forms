<script lang="ts">
  import { fly } from "svelte/transition"

	import type { Form } from "$lib/form/models/Form";

  export let form: Pick<Form, "title" | "edition" | "description">;
</script>

<header transition:fly={{ y: 50 }}>
  <h1>{form.title}</h1>
  <p class="edition">{form.edition ?? ""}</p>
  <p class="description">{form.description ?? ""}</p>
</header>

<style>
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
    max-width: min(95%,750px);
    margin-top: 2rem;
  }
</style>
