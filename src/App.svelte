<script lang="ts">
  let count = $state(0);

  function increment() {
    count += 1;
  }
</script>

<main>
  <h1>CommandRepo</h1>
  <div class="card">
    <button onclick={increment}>count is {count}</button>
  </div>
  <p>Build your command repository with Tauri + Svelte + TypeScript</p>
</main>

<style>
  main {
    padding: 2rem;
  }

  h1 {
    font-size: 2.5em;
    line-height: 1.1;
  }

  .card {
    padding: 2em;
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    background-color: #1a1a1a;
    cursor: pointer;
    transition: border-color 0.25s;
  }

  button:hover {
    border-color: #646cff;
  }

  button:focus,
  button:focus-visible {
    outline: 4px auto -webkit-focus-ring-color;
  }

  @media (prefers-color-scheme: light) {
    button {
      background-color: #f9f9f9;
    }
  }
</style>