<script lang="ts">
  import type { TaskDto } from '../api/types';
  import { formatBytes, formatEta, formatSpeed } from '../stores/format';

  export let tasks: TaskDto[] = [];
  export let selectedTaskId: string | null = null;
  export let onSelectTask: (taskId: string) => void = () => {};

  function progressPercent(progress: number) {
    return Math.max(0, Math.min(100, Math.round(progress * 100)));
  }

  function statusLabel(status: TaskDto['status']) {
    const labels: Record<TaskDto['status'], string> = {
      queued: '排队',
      checking: '校验',
      downloading: '下载中',
      paused: '暂停',
      completed: '完成',
      error: '错误'
    };
    return labels[status];
  }
</script>

<section class="task-table" aria-label="任务列表">
  <div class="head row">
    <span>名称</span>
    <span>状态</span>
    <span>进度</span>
    <span>下载</span>
    <span>上传</span>
    <span>剩余</span>
  </div>

  {#if tasks.length === 0}
    <div class="empty">暂无任务</div>
  {:else}
    {#each tasks as task}
      <button
        type="button"
        class:selected={task.id === selectedTaskId}
        class="row task-row"
        on:click={() => onSelectTask(task.id)}
      >
        <span class="name" title={task.name}>{task.name}</span>
        <span class={`status-badge ${task.status}`}>{statusLabel(task.status)}</span>
        <span class="progress-cell">
          <span>{progressPercent(task.progress)}% · {formatBytes(task.downloaded_bytes)} / {formatBytes(task.total_bytes)}</span>
          <span class="progress-track" aria-hidden="true">
            <span style={`width: ${progressPercent(task.progress)}%`}></span>
          </span>
        </span>
        <span class="speed down">{formatSpeed(task.download_speed_bps)}</span>
        <span class="speed up">{formatSpeed(task.upload_speed_bps)}</span>
        <span class="eta">{formatEta(task.eta_seconds)}</span>
      </button>
    {/each}
  {/if}
</section>

<style>
  .task-table {
    min-width: 0;
    overflow: auto;
    border: 1px solid var(--line);
    border-radius: 8px;
    background: rgb(13 20 27 / 0.86);
    box-shadow: var(--shadow);
  }

  .row {
    display: grid;
    grid-template-columns: minmax(190px, 1.5fr) 86px minmax(210px, 1.35fr) 98px 98px 74px;
    align-items: center;
    gap: 12px;
    width: 100%;
    min-height: 48px;
    padding: 0 16px;
  }

  .head {
    position: sticky;
    top: 0;
    z-index: 1;
    min-height: 42px;
    color: var(--muted);
    background: rgb(18 28 38 / 0.98);
    border-bottom: 1px solid var(--line);
    font-size: 12px;
    text-transform: uppercase;
  }

  .task-row {
    position: relative;
    border: 0;
    border-bottom: 1px solid rgb(36 53 68 / 0.72);
    border-radius: 0;
    color: var(--text);
    background: transparent;
    text-align: left;
  }

  .task-row:hover,
  .task-row.selected {
    background: linear-gradient(90deg, rgb(51 214 183 / 0.13), rgb(25 38 50 / 0.82));
  }

  .task-row.selected::before {
    position: absolute;
    inset: 0 auto 0 0;
    width: 3px;
    content: "";
    background: var(--accent);
    box-shadow: 0 0 14px rgb(51 214 183 / 0.7);
  }

  .name,
  .eta,
  .speed,
  .progress-cell > span:first-child {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .name {
    color: var(--text-strong);
    font-weight: 600;
  }

  .status-badge {
    justify-self: start;
    min-width: 58px;
    border: 1px solid rgb(141 164 182 / 0.24);
    border-radius: 999px;
    padding: 3px 8px;
    color: var(--muted);
    background: rgb(141 164 182 / 0.08);
    font-size: 12px;
    text-align: center;
  }

  .status-badge.downloading,
  .status-badge.checking {
    border-color: rgb(51 214 183 / 0.38);
    color: var(--accent-strong);
    background: rgb(51 214 183 / 0.1);
  }

  .status-badge.completed {
    border-color: rgb(112 227 146 / 0.38);
    color: var(--green);
    background: rgb(112 227 146 / 0.1);
  }

  .status-badge.error {
    border-color: rgb(255 127 115 / 0.38);
    color: var(--red);
    background: rgb(255 127 115 / 0.1);
  }

  .progress-cell {
    display: grid;
    gap: 6px;
    min-width: 0;
  }

  .progress-track {
    display: block;
    height: 4px;
    overflow: hidden;
    border-radius: 999px;
    background: #0a1117;
  }

  .progress-track span {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: linear-gradient(90deg, var(--blue), var(--accent));
    box-shadow: 0 0 12px rgb(51 214 183 / 0.42);
  }

  .speed.down {
    color: var(--accent-strong);
  }

  .speed.up {
    color: #9fcfff;
  }

  .empty {
    display: grid;
    place-items: center;
    min-height: 220px;
    color: var(--muted);
  }
</style>
