export const defaultApiBase = "http://127.0.0.1:8787";

const apiBaseStorageKey = "minggui.apiBase";

export function loadApiBase() {
  return localStorage.getItem(apiBaseStorageKey);
}

export function saveApiBase(apiBase) {
  localStorage.setItem(apiBaseStorageKey, apiBase);
}

