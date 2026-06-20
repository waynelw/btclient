<script lang="ts">
  import { defaultSettings } from '../stores/settings';

  export let open = false;
  export let onClose: () => void = () => {};
</script>

{#if open}
  <div class="overlay">
    <section class="dialog" aria-label="设置">
      <header>
        <h2>设置</h2>
        <button type="button" on:click={onClose}>关闭</button>
      </header>
      <label>
        <span>下载目录</span>
        <input value={defaultSettings.default_download_dir} />
      </label>
      <label>
        <span>刷新间隔</span>
        <input value={`${defaultSettings.status_refresh_ms} ms`} />
      </label>
    </section>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    display: grid;
    place-items: center;
    background: rgb(3 8 12 / 0.68);
    backdrop-filter: blur(10px);
  }

  .dialog {
    width: 420px;
    padding: 16px;
    border: 1px solid var(--line-strong);
    border-radius: 8px;
    background: var(--panel-strong);
    box-shadow: var(--shadow);
  }

  header,
  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  header {
    margin-bottom: 14px;
  }

  h2 {
    margin: 0;
    color: var(--text-strong);
    font-size: 16px;
  }

  label + label {
    margin-top: 10px;
  }

  input {
    width: 240px;
    min-height: 32px;
    padding: 0 10px;
  }
</style>
