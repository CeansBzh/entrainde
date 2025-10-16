import { invoke } from "@tauri-apps/api/core";
import { initAlerts, showSuccess, showError } from "./alerts";

let taskInputEl: HTMLInputElement | null;
let suggestionsEl: HTMLElement | null;

// Add a new task
async function addTask(name: string): Promise<void> {
  if (!name.trim()) return;

  try {
    await invoke("add_task", { name: name.trim() });
    showSuccess(`Tâche enregistrée : "${name}"`);
  } catch (error) {
    console.error("Error adding task:", error);
    showError("Erreur lors de l'enregistrement");
  }
}

// Search for similar tasks
async function searchSimilarTasks(query: string): Promise<string[]> {
  if (!query.trim()) return [];

  try {
    return await invoke<string[]>("search_tasks", { query });
  } catch (error) {
    console.error("Error searching tasks:", error);
    return [];
  }
}

// Display suggestions
async function showSuggestions(query: string): Promise<void> {
  if (!suggestionsEl) return;

  if (!query.trim()) {
    suggestionsEl.innerHTML = "";
    suggestionsEl.style.display = "none";
    return;
  }

  const suggestions = await searchSimilarTasks(query);

  if (suggestions.length === 0) {
    suggestionsEl.innerHTML = "";
    suggestionsEl.style.display = "none";
    return;
  }

  suggestionsEl.innerHTML = "";
  suggestions.slice(0, 5).forEach((suggestion) => {
    const div = document.createElement("div");
    div.className = "suggestion-item";
    div.textContent = suggestion;
    div.addEventListener("click", () => {
      if (taskInputEl && suggestionsEl) {
        taskInputEl.value = suggestion;
        suggestionsEl.innerHTML = "";
        suggestionsEl.style.display = "none";
        taskInputEl.focus();
      }
    });
    if (suggestionsEl) {
      suggestionsEl.appendChild(div);
    }
  });

  suggestionsEl.style.display = "block";
}

// Handle form submission
async function handleSubmit(e: Event): Promise<void> {
  e.preventDefault();

  if (!taskInputEl) return;

  const taskName = taskInputEl.value.trim();
  if (!taskName) return;

  await addTask(taskName);

  taskInputEl.value = "";
  if (suggestionsEl) {
    suggestionsEl.innerHTML = "";
    suggestionsEl.style.display = "none";
  }
}

// Open timeline window
async function openTimeline(): Promise<void> {
  try {
    await invoke("open_timeline");
  } catch (error) {
    console.error("Error opening timeline:", error);
    showError("Erreur lors de l'ouverture de l'historique");
  }
}

// Initialize the app
window.addEventListener("DOMContentLoaded", () => {
  taskInputEl = document.querySelector("#task-input");
  suggestionsEl = document.querySelector("#suggestions");
  
  // Initialize alerts
  const statusMsgEl = document.querySelector<HTMLElement>("#status-msg");
  initAlerts(statusMsgEl);

  // Handle form submission
  document.querySelector("#task-form")?.addEventListener("submit", handleSubmit);

  // Handle timeline button click
  document.querySelector("#timeline-btn")?.addEventListener("click", openTimeline);

  // Handle input changes for suggestions
  taskInputEl?.addEventListener("input", (e) => {
    const target = e.target as HTMLInputElement;
    showSuggestions(target.value);
  });

  // Hide suggestions when clicking outside
  document.addEventListener("click", (e) => {
    if (
      suggestionsEl &&
      taskInputEl &&
      e.target !== taskInputEl &&
      !suggestionsEl.contains(e.target as Node)
    ) {
      suggestionsEl.innerHTML = "";
      suggestionsEl.style.display = "none";
    }
  });
});
