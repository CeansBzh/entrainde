let statusMsgEl: HTMLElement | null = null;

// Initialize the status message element
export function initAlerts(element: HTMLElement | null): void {
  statusMsgEl = element;
}

// Show a success message
export function showSuccess(message: string, duration: number = 2000): void {
  if (!statusMsgEl) return;

  statusMsgEl.textContent = message;
  statusMsgEl.style.color = "";
  
  setTimeout(() => {
    if (statusMsgEl) statusMsgEl.textContent = "";
  }, duration);
}

// Show an error message
export function showError(message: string, duration: number = 3000): void {
  if (!statusMsgEl) return;

  statusMsgEl.textContent = message;
  statusMsgEl.style.color = "#f44336";
  
  setTimeout(() => {
    if (statusMsgEl) {
      statusMsgEl.textContent = "";
      statusMsgEl.style.color = "";
    }
  }, duration);
}

// Clear the status message
export function clearStatus(): void {
  if (!statusMsgEl) return;
  
  statusMsgEl.textContent = "";
  statusMsgEl.style.color = "";
}
