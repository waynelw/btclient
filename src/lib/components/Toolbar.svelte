<script lang="ts">
  import type { TaskDto } from '../api/types';

  export let selectedTask: TaskDto | null = null;
  export let onAddTorrent: (torrentPath: string) => void | Promise<void> = () => {};
  export let onStartTask: (taskId: string) => void | Promise<void> = () => {};
  export let onPauseTask: (taskId: string) => void | Promise<void> = () => {};
  export let onRemoveTask: (taskId: string) => void | Promise<void> = () => {};
  export let onOpenSettings: () => void = () => {};

  let fileInput: HTMLInputElement | null = null;
  let torrentSource = '';

  function openTorrentPicker() {
    fileInput?.click();
  }

  async function addTorrentSource() {
    const source = torrentSource.trim();
    await onAddTorrent(source);
    if (source) {
      torrentSource = '';
    }
  }

  async function handleTorrentFileChange(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0] as (File & { path?: string }) | undefined;
    await onAddTorrent(file?.path ?? '');
    input.value = '';
  }
</script>

<header class="toolbar">
  <div class="brand">
    <span class="brand-mark">BT</span>
    <div class="brand-copy">
      <strong>btclient</strong>
      <span>Rust BitTorrent 控制台</span>
    </div>
  </div>

  <div class="controls">
    <div class="source">
      <input
        bind:this={fileInput}
        class="torrent-input"
        type="file"
        accept=".torrent,application/x-bittorrent"
        on:change={handleTorrentFileChange}
      />
      <input
        class="source-input"
        bind:value={torrentSource}
        placeholder="torrent 路径或 magnet"
        on:keydown={(event) => {
          if (event.key === 'Enter') {
            void addTorrentSource();
          }
        }}
      />
      <button class="primary" type="button" on:click={addTorrentSource}>添加 torrent</button>
      <button type="button" on:click={openTorrentPicker}>选择文件</button>
    </div>

    <div class="actions">
      <button
        type="button"
        disabled={!selectedTask}
        on:click={() => selectedTask && onStartTask(selectedTask.id)}
      >
        开始
      </button>
      <button
        type="button"
        disabled={!selectedTask}
        on:click={() => selectedTask && onPauseTask(selectedTask.id)}
      >
        暂停
      </button>
      <button
        class="danger"
        type="button"
        disabled={!selectedTask}
        on:click={() => selectedTask && onRemoveTask(selectedTask.id)}
      >
        删除
      </button>
      <button class="ghost" type="button" on:click={onOpenSettings}>设置</button>
    </div>
  </div>
</header>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    min-width: 0;
    padding: 12px 16px;
    background:
      linear-gradient(180deg, rgb(21 31 42 / 0.96), rgb(13 20 27 / 0.94)),
      var(--panel);
    border-bottom: 1px solid rgb(124 247 215 / 0.12);
    box-shadow: 0 12px 32px rgb(0 0 0 / 0.22);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 210px;
  }

  .brand-mark {
    display: grid;
    place-items: center;
    width: 38px;
    height: 38px;
    border: 1px solid rgb(51 214 183 / 0.62);
    border-radius: 8px;
    color: var(--accent-strong);
    background: rgb(51 214 183 / 0.08);
    box-shadow: inset 0 0 18px rgb(51 214 183 / 0.08);
    font-size: 13px;
    font-weight: 800;
    letter-spacing: 0;
  }

  .brand-copy {
    display: grid;
    gap: 1px;
  }

  .brand-copy strong {
    color: var(--text-strong);
    font-size: 16px;
    letter-spacing: 0;
  }

  .brand-copy span {
    color: var(--muted);
    font-size: 12px;
  }

  .controls {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 12px;
    min-width: 0;
    flex: 1;
  }

  .source,
  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .torrent-input {
    display: none;
  }

  .source-input {
    width: min(34vw, 360px);
    min-width: 220px;
    height: 32px;
    padding: 0 10px;
  }

  @media (max-width: 1040px) {
    .brand-copy span {
      display: none;
    }

    .brand {
      min-width: 128px;
    }

    .source-input {
      min-width: 180px;
      width: 28vw;
    }
  }
</style>
