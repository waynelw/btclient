<script lang="ts">
  export let message: string | null = null;
  export let onDismiss: () => void = () => {};
</script>

{#if message}
  <div class="toast" role="alert">
    <span>{message}</span>
    <button type="button" on:click={onDismiss}>关闭</button>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    right: 16px;
    bottom: 44px;
    display: flex;
    align-items: center;
    gap: 12px;
    max-width: 420px;
    padding: 10px 12px;
    border: 1px solid #7f3b32;
    border-radius: 8px;
    color: #ffddd7;
    background: #2b1716;
  }
</style>
