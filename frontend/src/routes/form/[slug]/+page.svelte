<script lang="ts">
  import type { Form } from "$lib/form/models/Form.d"
  import type { Answer } from "$lib/form/models/Answer.d"

	import Layout from "$lib/presentation/Layout.svelte";
  
	import FormPage from "./FormPage.svelte";
	import Login from "./Login.svelte";

  export let data: {form: Form, user: string | null, API_HOST: string; answers: Answer[]; };
</script>

<Layout>
  {#if !data.user}
    <Login form={data.form}/>
  {:else}
    <FormPage {...data} />
  {/if}
</Layout>

<style>
  :global(body) {
    height: 100vh;
  }
</style>
