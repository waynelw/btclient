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
    <span>下载 {formatSpeed(totalDown)}</span>
    <span>上传 {formatSpeed(totalUp)}</span>
    <span>活跃任务 {activeTask ? activeTask.name : '无'}</span>
  </footer>

  <SettingsDialog open={settingsOpen} onClose={() => (settingsOpen = false)} />
  <ErrorToast message={errorMessage} onDismiss={onDismissError} />
</div>

<style>
  .shell {
    display: grid;
    grid-template-rows: 48px minmax(0, 1fr) 32px;
    height: 100vh;
    background: #101418;
    color: #d8dee9;
  }

  .workspace {
    display: grid;
    grid-template-columns: minmax(560px, 1fr) 340px;
    min-height: 0;
    border-top: 1px solid #25303d;
    border-bottom: 1px solid #25303d;
  }

  .statusbar {
    display: flex;
    align-items: center;
    gap: 18px;
    padding: 0 16px;
    color: #9fb0c3;
    background: #131a22;
  }
</style>
