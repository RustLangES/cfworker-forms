<script>
	import { page } from "$app/stores";
	import ButtonLink from "$lib/presentation/ButtonLink.svelte";
	import Layout from "$lib/presentation/Layout.svelte";
	import TopTitle from "$lib/presentation/TopTitle.svelte";

  const status = $page.status;
  const message = $page.error?.message;

</script>

<Layout>
  <TopTitle> {status} </TopTitle>

  <h2>{message}</h2>

  <ButtonLink href="/"> Go Back </ButtonLink>
</Layout>

<style>
  h2 {
    font-size: 2rem;
    margin-bottom: 2rem;

    text-transform: capitalize;
  }
</style>
