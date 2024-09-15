<script lang="ts">
	import type { Form } from "$lib/form/models/Form";

	import Button from "$lib/presentation/Button.svelte";
	import ButtonLink from "$lib/presentation/ButtonLink.svelte";
	import Header from "$lib/presentation/Header.svelte";

  export let playing: boolean;
  export let form: Form;
</script>

<Header {form} />

<Button on:click={() => playing = true}> Start </Button>

<ButtonLink href="/"> Go Back </ButtonLink>
