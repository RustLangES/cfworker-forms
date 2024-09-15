<script lang="ts">
  import type { Form } from "$lib/form/models/Form.d"
  import type { Answer } from "$lib/form/models/Answer.d"

	import Layout from "$lib/presentation/Layout.svelte";
  
	import IntroFormPage from "./IntroFormPage.svelte";
	import FormPage from "./FormPage.svelte";
	import Login from "./Login.svelte";

  export let data: {form: Form, user: string | null, API_HOST: string; answers: Answer[]; };

  let playing = (typeof window !== "undefined") && !!window.localStorage.getItem(`playing-${data.form.id}`);

  $: {
    if (typeof window !== "undefined") {
      if (playing) {
         window.localStorage.setItem(`playing-${data.form.id}`, "1")
      } else {
         window.localStorage.removeItem(`playing-${data.form.id}`)
      }
    }
  }
</script>

<Layout>
  {#if !data.user}
    <Login form={data.form}/>
  {:else if !playing}
    <IntroFormPage bind:playing form={data.form} />
  {:else}
    <FormPage {...data} bind:playing />
  {/if}
</Layout>

<style>
  :global(body) {
    height: 100vh;
  }
</style>
