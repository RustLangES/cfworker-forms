<script lang="ts">
	import type { Form } from "$lib/form/models/Form";

	import ButtonLink from "$lib/presentation/ButtonLink.svelte";
	import Header from "$lib/presentation/Header.svelte";

  export let form: Form;
</script>

<Header {form} />

<h3>This form require external login.</h3>

<ButtonLink href={`/login?form=${form.id}`}> GITHUB </ButtonLink>

<ButtonLink href="/"> Go Back </ButtonLink>

<style>
  :global(body) {
    display: grid;
    place-items: center;
  }
</style>
