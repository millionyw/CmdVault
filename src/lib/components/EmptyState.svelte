<script lang="ts">
  interface Props {
    oncreate?: () => void;
  }

  let { oncreate }: Props = $props();
</script>

<div class="empty-state">
  <div class="empty-icon">\u{1F4CB}</div>
  <h3>\u{6682}\u{65E0}\u{547D}\u{4EE4}</h3>
  <p class="hint">\u{5FEB}\u{901F}\u{6DFB}\u{52A0}\u{5E38}\u{7528}\u{547D}\u{4EE4}\u{FF0C}\u{65B9}\u{4FBF}\u{968F}\u{65F6}\u{8C03}\u{7528}</p>

  {#if oncreate}
    <button class="create-btn" onclick={oncreate}>
      \u{521B}\u{5EFA}\u{793A}\u{4F8B}\u{547D}\u{4EE4}
    </button>
  {/if}

  <p class="shortcut-hint">Ctrl+N \u{65B0}\u{5EFA}\u{547D}\u{4EE4}</p>
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 1rem;
    text-align: center;
    color: var(--text-secondary, #888);
  }

  .empty-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  h3 {
    font-size: 1.25rem;
    font-weight: 500;
    margin: 0 0 0.5rem 0;
    color: var(--text-primary, #fff);
  }

  .hint {
    font-size: 0.9rem;
    margin: 0 0 1.5rem 0;
    max-width: 250px;
  }

  .create-btn {
    background: var(--primary, #3b82f6);
    color: white;
    border: none;
    padding: 0.6rem 1.2rem;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background-color 0.15s ease;
    margin-bottom: 1rem;
  }

  .create-btn:hover {
    background: var(--primary-hover, #2563eb);
  }

  .shortcut-hint {
    font-size: 0.8rem;
    color: var(--text-muted, #666);
    margin: 0;
    padding: 0.3rem 0.6rem;
    background: var(--bg-tertiary, #1a1a1a);
    border-radius: 4px;
  }
</style>