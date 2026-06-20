<script lang="ts">
  import type { TaskDto } from '../api/types';
  import ErrorToast from './ErrorToast.svelte';
  import SettingsDialog from './SettingsDialog.svelte';
  import TaskDetail from './TaskDetail.svelte';
  import TaskTable from './TaskTable.svelte';
  import Toolbar from './Toolbar.svelte';
  import { formatSpeed } from '../stores/format';

  export let tasks: TaskDto[] = [];
  export let selectedTask: TaskDto | null = null;
  export let selectedTaskId: string | null = null;
  export let activeTask: TaskDto | null = null;
  export let errorMessage: string | null = null;
  export let onSelectTask: (taskId: string) => void = () => {};
  export let onAddTorrent: (torrentPath: string) => void | Promise<void> = () => {};
  export let onStartTask: (taskId: string) => void | Promise<void> = () => {};
  export let onPauseTask: (taskId: string) => void | Promise<void> = () => {};
  export let onRemoveTask: (taskId: string) => void | Promise<void> = () => {};
  export let onDismissError: () => void = () => {};

  let settingsOpen = false;

  $: totalDown = tasks.reduce((sum, task) => sum + task.download_speed_bps, 0);
  $: totalUp = tasks.reduce((sum, task) => sum + task.upload_speed_bps, 0);
</script>

<div class="shell">
  <Toolbar
    selectedTask={selectedTask}
    {onAddTorrent}
    {onStartTask}
    {onPauseTask}
    {onRemoveTask}
    onOpenSettings={() => (settingsOpen = true)}
  />

  <main class="workspace">
    <TaskTable {tasks} {selectedTaskId} {onSelectTask} />
    <TaskDetail task={selectedTask} />
  </main>

  <footer class="statusbar">
    <span class="pulse" aria-hidden="true"></span>
    <span>下载 <strong>{formatSpeed(totalDown)}</strong></span>
    <span>上传 <strong>{formatSpeed(totalUp)}</strong></span>
    <span class="active">活跃任务 <strong>{activeTask ? activeTask.name : '无'}</strong></span>
  </footer>

  <SettingsDialog open={settingsOpen} onClose={() => (settingsOpen = false)} />
  <ErrorToast message={errorMessage} onDismiss={onDismissError} />
</div>

<style>
  .shell {
    display: grid;
    grid-template-rows: 72px minmax(0, 1fr) 34px;
    height: 100vh;
    color: var(--text);
  }

  .workspace {
    display: grid;
    grid-template-columns: minmax(580px, 1fr) 360px;
    gap: 12px;
    min-height: 0;
    padding: 12px;
    border-top: 1px solid var(--line);
    border-bottom: 1px solid var(--line);
  }

  .statusbar {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 0 16px;
    overflow: hidden;
    color: var(--muted);
    background: rgb(10 15 20 / 0.88);
    border-top: 1px solid rgb(124 247 215 / 0.08);
  }

  strong {
    color: var(--text-strong);
    font-weight: 600;
  }

  .pulse {
    width: 7px;
    height: 7px;
    border-radius: 999px;
    background: var(--accent);
    box-shadow: 0 0 12px rgb(51 214 183 / 0.72);
  }

  .active {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
