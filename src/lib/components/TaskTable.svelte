<script lang="ts">
  import type { TaskDto } from '../api/types';
  import { formatBytes, formatEta, formatSpeed } from '../stores/format';

  export let tasks: TaskDto[] = [];
  export let selectedTaskId: string | null = null;
  export let onSelectTask: (taskId: string) => void = () => {};
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
        <span>{task.name}</span>
        <span>{task.status}</span>
        <span>{Math.round(task.progress * 100)}% · {formatBytes(task.downloaded_bytes)} / {formatBytes(task.total_bytes)}</span>
        <span>{formatSpeed(task.download_speed_bps)}</span>
        <span>{formatSpeed(task.upload_speed_bps)}</span>
        <span>{formatEta(task.eta_seconds)}</span>
      </button>
    {/each}
  {/if}
</section>

<style>
  .task-table {
    min-width: 0;
    overflow: auto;
    background: #101418;
  }

  .row {
    display: grid;
    grid-template-columns: minmax(180px, 1.6fr) 92px minmax(180px, 1.4fr) 100px 100px 80px;
    align-items: center;
    gap: 10px;
    width: 100%;
    min-height: 40px;
    padding: 0 14px;
  }

  .head {
    position: sticky;
    top: 0;
    color: #9fb0c3;
    background: #18212b;
    border-bottom: 1px solid #25303d;
  }

  .task-row {
    border: 0;
    border-bottom: 1px solid #202a35;
    border-radius: 0;
    color: #d8dee9;
    background: transparent;
    text-align: left;
  }

  .task-row:hover,
  .task-row.selected {
    background: #1c2834;
  }

  .empty {
    padding: 32px 16px;
    color: #8b9bad;
  }
</style>
