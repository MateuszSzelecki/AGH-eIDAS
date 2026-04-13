// At the moment I don't know how the user data would look, storing string as a placeholder
// There won't be a lot of data, so there is no database
// Using expo-secure-store which stores keypairs in systems Keystore, probably subject to change
// Functions should be used after authentication
import * as SecureStore from "expo-secure-store";

export interface UserId {
  dateBirth: number;
  dateIssue: number;
  firstName: string;
  lastName: string;
}

function validateUserId(obj: UserId): void {
  if (!obj) {
    throw new Error("UserId object is required");
  }

  if (!obj.firstName || typeof obj.firstName !== "string") {
    throw new Error("firstName is required and must be a string");
  }

  if (!obj.lastName || typeof obj.lastName !== "string") {
    throw new Error("lastName is required and must be a string");
  }

  if (typeof obj.dateBirth !== "number" || !Number.isFinite(obj.dateBirth)) {
    throw new Error("dateBirth is required and must be a number");
  }

  if (typeof obj.dateIssue !== "number" || !Number.isFinite(obj.dateIssue)) {
    throw new Error("dateIssue is required and must be a number");
  }

  if (obj.dateIssue < obj.dateBirth) {
    throw new Error("dateIssue cannot be earlier than dateBirth");
  }
}

export function userIdToString(user: UserId): string {
  validateUserId(user);
  return JSON.stringify(user);
}

export function createUserId(data: Partial<UserId>): UserId {
  if (!data) {
    throw new Error("Input data is required");
  }

  const requiredFields: (keyof UserId)[] = [
    "firstName",
    "lastName",
    "dateBirth",
    "dateIssue",
  ];

  for (const field of requiredFields) {
    if (data[field] === undefined || data[field] === null) {
      throw new Error(`Missing required field: ${field}`);
    }
  }

  const user: UserId = {
    firstName: data.firstName!,
    lastName: data.lastName!,
    dateBirth: data.dateBirth!,
    dateIssue: data.dateIssue!,
  };

  validateUserId(user);
  return user;
}

export function parseUserId(str: string): UserId {
  if (!str || typeof str !== "string") {
    throw new Error("Input must be a valid JSON string");
  }

  let parsed: unknown;

  try {
    parsed = JSON.parse(str);
  } catch {
    throw new Error("Invalid JSON format");
  }

  const user = parsed as UserId;

  validateUserId(user);
  return user;
}

export async function storeUserData(userData: UserId): Promise<void> {
  let userDataStr = userIdToString(userData);
  await SecureStore.setItemAsync("user-data", userDataStr);
}

export async function loadUserData(): Promise<UserId | null> {
  let userData = await SecureStore.getItemAsync("user-data");
  if (userData) {
    return parseUserId(userData);
  }
  return null;
}

export async function deleteUserData(): Promise<void> {
  await SecureStore.deleteItemAsync("user-data");
}
