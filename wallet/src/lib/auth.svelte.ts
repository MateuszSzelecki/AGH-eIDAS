import { invoke } from "@tauri-apps/api/core";

export let user = $state({
  user: false,
});

export async function login(username: string, password: string) {
  await invoke("login", { username: username, password: password });
  checkAuth();
}

export async function register(username: string, password: string) {
  await invoke("register", { username: username, password: password });
  login(username, password);
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
