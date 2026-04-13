import React, { createContext, useContext, useEffect, useState } from "react";

import { Redirect, useSegments } from "expo-router";
import * as SecureStore from "expo-secure-store";

import * as storageService from "@/services/storageService";

import * as LocalAuthentication from "expo-local-authentication";
type User = {
  token: string;
};

type AuthContextType = {
  user: User | null;
  loading: boolean;
  login: (token: string) => Promise<void>;
  logout: () => Promise<void>;
  biometricsCheck: (message: string) => Promise<boolean>;
  getUserData: (message: string) => Promise<storageService.UserId | null>;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const segments = useSegments();

  useEffect(() => {
    loadUser();
  }, []);

  const loadUser = async (): Promise<void> => {
    const token = await SecureStore.getItemAsync("token");
    if (token) {
      setUser({ token });
    }
    setLoading(false);
  };

  const register = async (): Promise<void> => {
    // TO DO: Register user
    login("fake-token");
    return;
  };

  const login = async (token: string): Promise<void> => {
    // TO DO: Login user
    await SecureStore.setItemAsync("token", token);
    let userData: storageService.UserId = storageService.createUserId({
      firstName: "test",
      lastName: "test",
      dateBirth: 1,
      dateIssue: Date.now(),
    });
    await storageService.storeUserData(userData);
    setUser({ token });
  };

  const logout = async (): Promise<void> => {
    await SecureStore.deleteItemAsync("token");
    await storageService.deleteUserData();
    setUser(null);
  };

  const biometricsCheck = async (message: string): Promise<boolean> => {
    console.log("Initiating biometrics check");

    const hasHardware = await LocalAuthentication.hasHardwareAsync();
    const isEnrolled = await LocalAuthentication.isEnrolledAsync();

    if (!hasHardware || !isEnrolled) {
      console.log("No biometric hardware avaible, or not enrolled");
      return false;
    }

    const result = await LocalAuthentication.authenticateAsync({
      promptMessage: message,
    });

    return result.success;
  };

  const getUserData = async (
    message: string,
  ): Promise<storageService.UserId | null> => {
    if (!user) {
      return null;
    }
    if (await biometricsCheck(message)) {
      return storageService.loadUserData();
    }
    return null;
  };

  const inAuth = segments[0] === "(auth)";
  if (!user && !inAuth) {
    console.log("Redirecting to auth");
    return <Redirect href="/(auth)/" />;
  }

  console.log("Returning default auth provider");
  return (
    <AuthContext.Provider
      value={{ user, loading, login, logout, biometricsCheck, getUserData }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used inside AuthProvider");
  }
  return context;
}
