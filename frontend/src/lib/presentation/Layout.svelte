<script>
	import { fade } from "svelte/transition";

	import Footer from "$lib/presentation/Footer.svelte";

  export let container = false;
</script>

<main transition:fade class:container>
  <slot />
</main>

<Footer />

<style>
  :global(body) {
    height: 100vh;
    overflow-y: hidden;
  }

  main {
    height: 100vh;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  main.container {
    width: 100vw;
    max-width: 700px;

    display: grid;
    grid-template-rows: 1fr 2fr;

    flex-direction: initial;
    justify-content: initial;
    align-items: initial;
  }
</style>

