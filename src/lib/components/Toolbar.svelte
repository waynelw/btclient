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
  <div class="left">
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
    <button type="button" on:click={addTorrentSource}>添加 torrent</button>
    <button type="button" on:click={openTorrentPicker}>选择文件</button>
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
      type="button"
      disabled={!selectedTask}
      on:click={() => selectedTask && onRemoveTask(selectedTask.id)}
    >
      删除
    </button>
  </div>
  <button type="button" on:click={onOpenSettings}>设置</button>
</header>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 12px;
    background: #151d26;
  }

  .left {
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
    border: 1px solid #344254;
    border-radius: 6px;
    padding: 0 10px;
    color: #d8dee9;
    background: #101820;
  }
</style>
