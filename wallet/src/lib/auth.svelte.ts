import { invoke } from "@tauri-apps/api/core";

export let user = $state({
  user: false,
});

export function getIssuerUrl() {
  let hostname = "localhost";
  if (typeof window !== "undefined" && window.location.hostname) {
    const hn = window.location.hostname;
    // In production builds, Tauri assets are served from tauri.localhost
    if (hn !== "tauri.localhost") {
      hostname = hn;
    }
  }
  return `http://${hostname}:8000`;
}

export async function login(username: string, password: string) {
  try {
    const issuerUrl = getIssuerUrl();
    await invoke("login", { username, password, issuerUrl });
    await checkAuth();
  } catch (error) {
    console.error("Login error:", error);
    throw error;
  }
}

export async function register(username: string, email: string, officeCode: string, password: string) {
  try {
    const issuerUrl = getIssuerUrl();
    await invoke("register", { username, email, officeCode, password, issuerUrl });
    await login(username, password);
  } catch (error) {
    console.error("Registration error:", error);
    throw error;
  }
}

export async function checkAuth() {
  try {
    const currentUser: boolean = await invoke("is_auth");
    user.user = currentUser;
  } catch {
    user.user = false;
  }
}

export async function logout() {
  await invoke("logout");
  await checkAuth();
}
