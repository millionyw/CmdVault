<script lang="ts">
  interface Props {
    value: string;
    disabled?: boolean;
    onchange?: (value: string) => void;
  }

  let { value, disabled = false, onchange }: Props = $props();

  let recording = $state(false);
  let buttonRef: HTMLButtonElement | undefined = $state();

  function startRecording() {
    if (disabled) return;
    recording = true;
    buttonRef?.focus();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!recording) return;
    e.preventDefault();
    e.stopPropagation();

    // Escape cancels recording
    if (e.key === 'Escape') {
      recording = false;
      return;
    }

    // Build shortcut string in Tauri format
    const parts: string[] = [];
    if (e.ctrlKey || e.metaKey) parts.push('CommandOrControl');
    if (e.shiftKey) parts.push('Shift');
    if (e.altKey) parts.push('Alt');

    // Get main key
    let key = e.key;
    if (key.length === 1) {
      key = key.toUpperCase();
    } else if (key === ' ') {
      key = 'Space';
    } else if (key.startsWith('Arrow')) {
      key = key.slice(5);
    }

    // Don't accept modifier-only shortcuts
    if (parts.length === 0 && ['Control', 'Shift', 'Alt', 'Meta'].includes(key)) {
      return;
    }

    parts.push(key);
    const shortcut = parts.join('+');

    recording = false;
    onchange?.(shortcut);
  }

  function handleBlur() {
    recording = false;
  }

  function formatDisplay(val: string): string {
    return val
      .replace(/CommandOrControl/gi, 'Ctrl')
      .replace(/\+/g, ' + ');
  }
</script>

<button
  bind:this={buttonRef}
  class="recorder {recording ? 'recording' : ''}"
  class:disabled
  onclick={startRecording}
  onkeydown={handleKeydown}
  onblur={handleBlur}
  disabled={disabled}
  type="button"
>
  {#if recording}
    <span class="hint">按下新快捷键...</span>
  {:else}
    <span class="value">{formatDisplay(value)}</span>
  {/if}
</button>

<style>
  .recorder {
    min-width: 120px;
    padding: 0.4rem 0.8rem;
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    color: #fff;
    font-size: 0.85rem;
    font-family: monospace;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: center;
  }

  .recorder:hover:not(.disabled) {
    background: #333;
    border-color: #555;
  }

  .recorder:focus {
    outline: none;
    border-color: var(--primary, #3b82f6);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }

  .recorder.recording {
    background: #3b82f6;
    border-color: #3b82f6;
  }

  .recorder.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .hint {
    color: rgba(255, 255, 255, 0.9);
  }

  .value {
    color: #fff;
  }
</style>