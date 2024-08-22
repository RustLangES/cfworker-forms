<script lang="ts">
	import type { Form } from "$lib/form/models/Form";

	import ButtonLink from "$lib/presentation/ButtonLink.svelte";
  import TopTitle from "$lib/presentation/TopTitle.svelte";

  export let form: Form;
</script>

<TopTitle> Login </TopTitle>

<h3>This form require external login.</h3>

<br/>

<ButtonLink href={`/login?form=${form.id}`}> GITHUB </ButtonLink>

<br/> <br/>

<ButtonLink href="/"> Go Back </ButtonLink>
