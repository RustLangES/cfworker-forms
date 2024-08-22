<script>
  import { fly } from "svelte/transition";
</script>

<h1 transition:fly|global={{ y: -50 }}>
  <slot />
</h1>

<style>
  h1 {
    font-size: 1.5rem;
    position: fixed;
    top: 1rem;
    left: 50vw;
    transform: translateX(-50%);

    font-family: var(--fonts-heading);
    font-weight: 400;
    color: var(--title);
  }
</style>
