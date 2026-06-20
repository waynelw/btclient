<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from './lib/components/AppShell.svelte';
  import type { TaskDto } from './lib/api/types';
  import {
    addTorrent,
    getAppSnapshot,
    pauseTask,
    removeTask as removeTaskCommand,
    startTask
  } from './lib/api/commands';
  import { defaultSettings } from './lib/stores/settings';
  import {
    removeTask as removeTaskFromState,
    selectTask,
    upsertTask
  } from './lib/stores/tasks';

  let tasks: TaskDto[] = [];
  let selectedTaskId: string | null = null;
  let errorMessage: string | null = null;
  let defaultDownloadDir = defaultSettings.default_download_dir;
  let statusRefreshMs = defaultSettings.status_refresh_ms;

  $: selectedTask = tasks.find((task) => task.id === selectedTaskId) ?? tasks[0] ?? null;
  $: activeTask = tasks.find((task) => task.status === 'downloading' || task.status === 'checking') ?? null;

  onMount(() => {
    let refreshTimer: ReturnType<typeof window.setInterval>;

    void refreshSnapshot();
    refreshTimer = window.setInterval(() => {
      void refreshSnapshot();
    }, statusRefreshMs);

    return () => window.clearInterval(refreshTimer);
  });

  async function refreshSnapshot() {
    try {
      const snapshot = await getAppSnapshot();
      tasks = snapshot.tasks;
      defaultDownloadDir = snapshot.settings.default_download_dir;
      statusRefreshMs = snapshot.settings.status_refresh_ms;
      reconcileSelection();
    } catch (error) {
      setError(error);
    }
  }

  async function handleAddTorrent(torrentPath: string) {
    if (!torrentPath) {
      errorMessage = '无法读取 torrent 文件路径';
      return;
    }

    try {
      const task = await addTorrent(torrentPath, defaultDownloadDir);
      tasks = upsertTask({ tasks, selectedTaskId }, task).tasks;
      selectedTaskId = task.id;
      errorMessage = null;
    } catch (error) {
      setError(error);
    }
  }

  async function handleStartTask(taskId: string) {
    try {
      const task = await startTask(taskId);
      tasks = upsertTask({ tasks, selectedTaskId }, task).tasks;
      selectedTaskId = task.id;
      errorMessage = null;
    } catch (error) {
      setError(error);
    }
  }

  async function handlePauseTask(taskId: string) {
    try {
      const task = await pauseTask(taskId);
      tasks = upsertTask({ tasks, selectedTaskId }, task).tasks;
      selectedTaskId = task.id;
      errorMessage = null;
    } catch (error) {
      setError(error);
    }
  }

  async function handleRemoveTask(taskId: string) {
    try {
      await removeTaskCommand(taskId, false);
      const state = removeTaskFromState({ tasks, selectedTaskId }, taskId);
      tasks = state.tasks;
      selectedTaskId = state.selectedTaskId;
      reconcileSelection();
      errorMessage = null;
    } catch (error) {
      setError(error);
    }
  }

  function handleSelectTask(taskId: string) {
    selectedTaskId = selectTask({ tasks, selectedTaskId }, taskId).selectedTaskId;
  }

  function reconcileSelection() {
    if (selectedTaskId && tasks.some((task) => task.id === selectedTaskId)) {
      return;
    }
    selectedTaskId = tasks[0]?.id ?? null;
  }

  function setError(error: unknown) {
    errorMessage = error instanceof Error ? error.message : String(error);
  }
</script>

<AppShell
  {tasks}
  {selectedTask}
  {selectedTaskId}
  {activeTask}
  {errorMessage}
  onSelectTask={handleSelectTask}
  onAddTorrent={handleAddTorrent}
  onStartTask={handleStartTask}
  onPauseTask={handlePauseTask}
  onRemoveTask={handleRemoveTask}
  onDismissError={() => (errorMessage = null)}
/>
