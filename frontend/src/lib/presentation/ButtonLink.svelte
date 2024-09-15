<script lang="ts">
  export let href: string;
</script>

<a {href} on:click>
  <slot />
</a>

<style>
  a {
    margin-top: 0.5rem;
    padding: 0.5rem 1.5rem;

    text-decoration: none;
    color: white;
    background: #2f2f2f;
    border: solid 1.5px #FFF;
    font-size: 1rem;
    font-family: var(--fonts-body);

    filter: var(--dec-shadow-filter);
    transition: filter 500ms ease;
  }

  a:hover {
    filter: drop-shadow(0px 0px 0px #000)
  }
</style>
