<script>
	import FormField from "$lib/form/FormField.svelte";

</script>

<h1>
  Rust Lang ES

  <span> Forms </span>
</h1>

<div>
  <FormField />
  <FormField />
  <FormField />

  <button> ABC </button>
</div>

<h1>
  Rust Lang ES

  <span> Forms </span>
</h1>

<style>
  :global(body) {
    display: grid;
    place-items: center;
    height: 100vh;
  }

  h1 {
    font-family: "AlfaSlabOne", "sans-serif";
    color: var(--title);
  }

  h1 {
    font-family: "AlfaSlabOne", "sans-serif";
  }

  button {
    color: white;
    border: solid 1.5px #FFF;
    background: transparent;
    filter: var(--dec-shadow-filter);
  }

  div {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
