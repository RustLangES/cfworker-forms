<script lang="ts">
	import type { Form } from "$lib/form/models/Form.d";

	import Header from "$lib/presentation/Header.svelte";
	import Layout from "$lib/presentation/Layout.svelte";

  export let data: { forms: Form[] };
</script>

<Layout container>
  <Header form={{
    title: "RustLangES",
    edition: "",
    description: ""
  }} />

  <ul>
    {#each data.forms as form}
      <li>
        <a href={`/form/${form.id}`}>
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

  ul {
    height: min-content;
    padding: 2rem;

    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  li {
    background: #2f2f2f;
    border: solid 1px #FFF;
    color: white;

    filter: var(--dec-shadow-filter);
    transition: filter 500ms ease;
  }

  li:hover {
    filter: drop-shadow(0px 0px 0px #000)
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
