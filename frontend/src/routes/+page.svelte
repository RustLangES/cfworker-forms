<script lang="ts">
  const forms = [
    {
      id: 1,
      title: "Form 1",
      description: "Description 1",
      edition: "",
    },
    {
      id: 2,
      title: "Event",
      description: "My awesome event",
      edition: "2024",
    }
  ];

  const defaultForm = {
    id: 0,
    title: "Rust Lang Es",
    edition: "",
    description: ""
  };

  let form = defaultForm;

  function mouseEnter(newForm: typeof defaultForm) {
    return () => {
      form = newForm;
    }
  }

  function mouseLeave() {
    if (form.id !== 0) {
      form = defaultForm;
    }
  }
</script>

<main>
  <header>
    <h1>{form.title}</h1>
    <p class="edition">{form.edition}</p>
    <p class="description">{form.description}</p>
  </header>

  <ul on:mouseleave={mouseLeave}>
    {#each forms as form}
      <li on:mouseenter={mouseEnter(form)}>
        <a href={`/form/${form.id}`}>
          <span>{form.title}</span>
          <span>&gt;</span>
        </a>
      </li>
    {/each}
  </ul>

  <footer> RustLangES Forms </footer>
</main>

<style>
  :global(body) {
    display: grid;
    place-items: center;
    height: 100vh;
  }

  main {
    width: 100vw;
    max-width: 700px;
    height: 100vh;

    display: grid;
    grid-template-rows: 1fr 2fr;
  }

  header {
    margin-top: max(2rem, 2vh);
    padding-top: max(2rem, 2vh);

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  h1 {
    font-size: 3rem;
    letter-spacing: 0.05ch;

    font-family: var(--fonts-heading);
    color: var(--title);
  }

  p {
    min-height: 2rem;
  }

  .edition {
    font-size: 1.25rem;
    font-family: var(--fonts-heading);
  }

  .description {
    margin-top: 2rem;
  }

  ul {
    height: min-content;
    padding: 2rem;

    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  li {
    border: solid 1px #FFF;
    color: white;

    filter: var(--dec-shadow-filter);
    transition: filter 500ms ease;
  }

  li:hover {
    filter: drop-shadow(0px 0px 0px #000)
  }

  a {
    width: 100%;
    height: 100%;
    padding: 1rem 2rem;

    color: currentColor;
    text-decoration: none;

    display: flex;
    justify-content: space-between;
  }

  footer {
    position: fixed;
    bottom: 1rem;
    left: 50vw;
    transform: translateX(-50%);

    font-size: 0.8rem;
    font-family: var(--fonts-heading);
    color: var(--title);
  }
</style>
