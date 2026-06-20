<script lang="ts">
  import type { TaskDto } from '../api/types';
  import { formatBytes, formatSpeed } from '../stores/format';

  export let task: TaskDto | null = null;
</script>

<aside class="detail" aria-label="任务详情">
  {#if task}
    <h2>{task.name}</h2>
    <dl>
      <dt>状态</dt>
      <dd>{task.status}</dd>
      <dt>下载目录</dt>
      <dd>{task.download_dir}</dd>
      <dt>大小</dt>
      <dd>{formatBytes(task.total_bytes)}</dd>
      <dt>下载速度</dt>
      <dd>{formatSpeed(task.download_speed_bps)}</dd>
      <dt>Peer</dt>
      <dd>{task.peers_connected} / {task.peers_total}</dd>
    </dl>
    {#if task.error}
      <p class="error">{task.error.message}</p>
    {/if}
  {:else}
    <p class="muted">未选择任务</p>
  {/if}
</aside>

<style>
  .detail {
    min-width: 0;
    padding: 16px;
    background: #131a22;
    border-left: 1px solid #25303d;
  }

  h2 {
    margin: 0 0 16px;
    font-size: 16px;
  }

  dl {
    display: grid;
    grid-template-columns: 82px minmax(0, 1fr);
    gap: 10px;
    margin: 0;
  }

  dt {
    color: #8b9bad;
  }

  dd {
    min-width: 0;
    margin: 0;
    overflow-wrap: anywhere;
  }

  .error {
    margin-top: 16px;
    color: #ffb4a8;
  }
</style>
