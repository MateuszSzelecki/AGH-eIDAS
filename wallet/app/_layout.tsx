import { Slot } from "expo-router";
import { AuthProvider } from "../context/AuthContext";

export default function RootLayout(): JSX.Element {
  return (
    <AuthProvider>
      <Slot />
    </AuthProvider>
  );
}
