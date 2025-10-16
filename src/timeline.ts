import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// DOM element IDs
const DOM_IDS = {
  TIMELINE_LIST: "#timeline-list",
  TIMELINE_LOADING: "#timeline-loading",
  TIMELINE_EMPTY: "#timeline-empty",
  REFRESH_BTN: "#refresh-btn"
} as const;

interface Task {
  name: string;
  timestamp: number;
}

// Format timestamp to human readable time
function formatTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleTimeString('fr-FR', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  });
}

// Get all tasks from the backend
async function getAllTasks(): Promise<Task[]> {
  try {
    return await invoke<Task[]>("get_tasks");
  } catch (error) {
    console.error("Error fetching tasks:", error);
    return [];
  }
}

function createTaskElement(task: Task): HTMLLIElement {
  const timelineItem = document.createElement("li");
  timelineItem.className = "timeline-item";

  const taskEl = document.createElement("p");
  taskEl.className = "timeline-task";
  taskEl.textContent = task.name;

  const timeEl = document.createElement("div");
  timeEl.className = "timeline-time";
  timeEl.textContent = formatTime(task.timestamp);

  timelineItem.appendChild(taskEl);
  timelineItem.appendChild(timeEl);

  return timelineItem;
}

// Render tasks in the timeline
function renderTasks(tasks: Task[]): void {
  const timelineList = document.querySelector(DOM_IDS.TIMELINE_LIST);
  const loadingEl = document.querySelector(DOM_IDS.TIMELINE_LOADING) as HTMLElement;
  const emptyEl = document.querySelector(DOM_IDS.TIMELINE_EMPTY) as HTMLElement;

  if (!timelineList || !loadingEl || !emptyEl) return;

  // Hide loading indicator
  loadingEl.style.display = "none";

  // Sort tasks chronologically (oldest first)
  const sortedTasks = tasks.sort((a, b) => a.timestamp - b.timestamp);

  if (sortedTasks.length === 0) {
    emptyEl.style.display = "flex";
    return;
  }

  // Hide empty message
  emptyEl.style.display = "none";

  // Clear previous content
  timelineList.innerHTML = "";

  // Render each task using shared function
  sortedTasks.forEach(task => {
    const taskElement = createTaskElement(task);
    timelineList.appendChild(taskElement);
  });
}

// Add a single task to the timeline (for real-time updates)
function addTaskToTimeline(task: Task): void {
  const timelineList = document.querySelector(DOM_IDS.TIMELINE_LIST);
  const emptyEl = document.querySelector(DOM_IDS.TIMELINE_EMPTY) as HTMLElement;

  if (!timelineList) {
    return;
  }

  // Hide empty message if it's showing
  if (emptyEl && emptyEl.style.display !== "none") {
    emptyEl.style.display = "none";
  }

  const timelineItem = createTaskElement(task);
  timelineList.appendChild(timelineItem);

  // Add a subtle animation to highlight the new task
  timelineItem.style.backgroundColor = "#e8f4fd";
  setTimeout(() => {
    timelineItem.style.backgroundColor = "";
  }, 2000);
}

// Initialize the timeline
async function initTimeline(): Promise<void> {
  try {
    const tasks = await getAllTasks();
    renderTasks(tasks);
  } catch (error) {
    console.error("Error initializing timeline:", error);

    const loadingEl = document.querySelector(DOM_IDS.TIMELINE_LOADING) as HTMLElement;
    if (loadingEl) {
      loadingEl.innerHTML = "<p>Erreur lors du chargement des tâches</p>";
    }
  }
}

window.addEventListener("DOMContentLoaded", () => {
  initTimeline();
});

listen<Task>("task-added", (event) => {
  addTaskToTimeline(event.payload);
});