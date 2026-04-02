// At the moment I don't know how the user data would look, storing string as a placeholder
// There won't be a lot of data, so there is no database
// Using expo-secure-store which stores keypairs in systems Keystore, probably subject to change
// Functions should be used after authentication
import * as SecureStore from "expo-secure-store";

export async function storeUserData(userData: string): Promise<void> {
  await SecureStore.setItemAsync("user-data", userData);
}

export async function loadUserData(): Promise<string | null> {
  return await SecureStore.getItemAsync("user-data");
}

export async function deleteUserData(): Promise<void> {
  await SecureStore.deleteItemAsync("user-data");
}
