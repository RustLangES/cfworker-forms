<button on:click> <slot /> </button>

<style>
  button {
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

  button:hover {
    filter: drop-shadow(0px 0px 0px #000)
  }
</style>

