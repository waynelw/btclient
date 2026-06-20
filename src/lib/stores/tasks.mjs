export function createTaskState(tasks = [], selectedTaskId = null) {
  return {
    tasks: [...tasks],
    selectedTaskId
  };
}

export function upsertTask(state, task) {
  const index = state.tasks.findIndex((item) => item.id === task.id);
  if (index === -1) {
    return {
      ...state,
      tasks: [...state.tasks, task]
    };
  }

  return {
    ...state,
    tasks: state.tasks.map((item, itemIndex) => (itemIndex === index ? task : item))
  };
}

export function selectTask(state, taskId) {
  return {
    ...state,
    selectedTaskId: taskId
  };
}

export function removeTask(state, taskId) {
  const tasks = state.tasks.filter((item) => item.id !== taskId);
  return {
    ...state,
    tasks,
    selectedTaskId: state.selectedTaskId === taskId ? null : state.selectedTaskId
  };
}
