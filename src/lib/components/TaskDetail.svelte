<script lang="ts">
  import type { TaskDto } from '../api/types';
  import { formatBytes, formatEta, formatSpeed } from '../stores/format';

  export let task: TaskDto | null = null;

  $: progress = task ? Math.max(0, Math.min(100, Math.round(task.progress * 100))) : 0;

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

<aside class="detail" aria-label="任务详情">
  {#if task}
    <header>
      <span class={`status-badge ${task.status}`}>{statusLabel(task.status)}</span>
      <h2>{task.name}</h2>
      <div class="progress-summary">
        <strong>{progress}%</strong>
        <span>{formatBytes(task.downloaded_bytes)} / {formatBytes(task.total_bytes)}</span>
      </div>
      <div class="progress-track" aria-hidden="true">
        <span style={`width: ${progress}%`}></span>
      </div>
    </header>

    <section class="metric-grid" aria-label="传输指标">
      <div>
        <span>下载</span>
        <strong>{formatSpeed(task.download_speed_bps)}</strong>
      </div>
      <div>
        <span>上传</span>
        <strong>{formatSpeed(task.upload_speed_bps)}</strong>
      </div>
      <div>
        <span>Peer</span>
        <strong>{task.peers_connected} / {task.peers_total}</strong>
      </div>
      <div>
        <span>剩余</span>
        <strong>{formatEta(task.eta_seconds)}</strong>
      </div>
    </section>

    <dl>
      <dt>下载目录</dt>
      <dd>{task.download_dir}</dd>
      <dt>Torrent</dt>
      <dd>{task.torrent_path}</dd>
      <dt>创建时间</dt>
      <dd>{task.created_at}</dd>
      <dt>更新时间</dt>
      <dd>{task.updated_at}</dd>
    </dl>
    {#if task.error}
      <p class="error">{task.error.message}</p>
    {/if}
  {:else}
    <div class="empty-detail">
      <strong>未选择任务</strong>
      <span>添加 torrent 后可查看传输指标与路径信息</span>
    </div>
  {/if}
</aside>

<style>
  .detail {
    min-width: 0;
    overflow: auto;
    padding: 18px;
    border: 1px solid var(--line);
    border-radius: 8px;
    background:
      linear-gradient(180deg, rgb(21 31 42 / 0.94), rgb(13 20 27 / 0.94)),
      var(--panel);
    box-shadow: var(--shadow);
  }

  header {
    padding-bottom: 18px;
    border-bottom: 1px solid var(--line);
  }

  h2 {
    margin: 12px 0 14px;
    color: var(--text-strong);
    font-size: 17px;
    line-height: 1.35;
    overflow-wrap: anywhere;
  }

  .status-badge {
    display: inline-flex;
    border: 1px solid rgb(51 214 183 / 0.36);
    border-radius: 999px;
    padding: 3px 9px;
    color: var(--accent-strong);
    background: rgb(51 214 183 / 0.1);
    font-size: 12px;
  }

  .status-badge.paused,
  .status-badge.queued {
    border-color: rgb(141 164 182 / 0.28);
    color: var(--muted);
    background: rgb(141 164 182 / 0.08);
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

  .progress-summary {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
    color: var(--muted);
  }

  .progress-summary strong {
    color: var(--text-strong);
    font-size: 28px;
    line-height: 1;
  }

  .progress-track {
    height: 7px;
    margin-top: 12px;
    overflow: hidden;
    border-radius: 999px;
    background: #081017;
  }

  .progress-track span {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: linear-gradient(90deg, var(--blue), var(--accent));
    box-shadow: 0 0 14px rgb(51 214 183 / 0.5);
  }

  .metric-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 10px;
    margin: 16px 0;
  }

  .metric-grid div {
    display: grid;
    gap: 3px;
    min-width: 0;
    padding: 12px;
    border: 1px solid rgb(36 53 68 / 0.82);
    border-radius: 8px;
    background: rgb(10 17 24 / 0.64);
  }

  .metric-grid span {
    color: var(--muted);
    font-size: 12px;
  }

  .metric-grid strong {
    min-width: 0;
    overflow: hidden;
    color: var(--text-strong);
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  dl {
    display: grid;
    grid-template-columns: 72px minmax(0, 1fr);
    gap: 10px 12px;
    margin: 0;
  }

  dt {
    color: var(--muted);
  }

  dd {
    min-width: 0;
    margin: 0;
    color: var(--text);
    overflow-wrap: anywhere;
  }

  .error {
    margin-top: 16px;
    border: 1px solid rgb(255 127 115 / 0.28);
    border-radius: 8px;
    padding: 10px 12px;
    color: #ffd7d2;
    background: rgb(255 127 115 / 0.1);
  }

  .empty-detail {
    display: grid;
    place-items: center;
    align-content: center;
    gap: 6px;
    min-height: 100%;
    color: var(--muted);
    text-align: center;
  }

  .empty-detail strong {
    color: var(--text-strong);
  }
</style>
