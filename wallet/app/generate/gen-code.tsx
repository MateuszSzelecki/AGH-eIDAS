import { View, Text, ActivityIndicator, Button } from "react-native";
import { useAuth } from "@/context/AuthContext";
import { useEffect, useState, useRef } from "react";
import { router } from "expo-router";
import { generateProof } from "@/services/zkpService";

import { StyleSheet } from "react-native";
import { ThemedText } from "@/components/themed-text";
import { ThemedView } from "@/components/themed-view";

export default function CodeScreen() {
  const { user, biometricsCheck, getUserData } = useAuth();
  const [loading, setLoading] = useState<boolean>(true);
  const [code, setCode] = useState<string | null>(null);
  const [authenticated, setAuthenticated] = useState<boolean>(false);
  const [timeLeft, setTimeLeft] = useState(30);
  const [expired, setExpired] = useState(false);

  const timerRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    if (!user) {
      router.replace("/(auth)/login");
      return;
    }
    authenticateAndGenerate();

    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, [user]);

  const startTimer = () => {
    setTimeLeft(30);
    setExpired(false);

    if (timerRef.current) clearInterval(timerRef.current);

    timerRef.current = setInterval(() => {
      setTimeLeft((prev) => {
        if (prev <= 1) {
          clearInterval(timerRef.current!);
          setExpired(true);
          setCode(null);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  };

  const authenticateAndGenerate = async () => {
    setLoading(true);

    const data = await generateProof(
      await getUserData("Generate verification code"),
    );

    if (data) {
      setAuthenticated(true);
      setCode(data);
      startTimer();
    }
    setLoading(false);
  };

  const handleRegenerate = async () => {
    await authenticateAndGenerate();
  };

  if (loading) {
    return <ActivityIndicator />;
  }

  if (!authenticated) {
    return (
      <View>
        <Text>User not authenticated</Text>
      </View>
    );
  }

  return (
    <ThemedView style={styles.container}>
      <ThemedText style={styles.title}>
        Use this code to verify your age:
      </ThemedText>

      {!expired && code && (
        <Text style={styles.timer}>Expires in: {timeLeft}s</Text>
      )}

      {code && !expired ? (
        <Text style={styles.code}>{code}</Text>
      ) : (
        <Text style={styles.expiredText}>Code expired</Text>
      )}
      <View style={styles.buttonContainer}>
        {expired && (
          <Button title="Regenerate Code" onPress={handleRegenerate} />
        )}

        <Button title="Back to Home" onPress={() => router.replace("/home")} />
      </View>
    </ThemedView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    padding: 20,
  },
  title: {
    marginBottom: 20,
    fontSize: 18,
    fontWeight: "600",
  },
  buttonContainer: {
    marginTop: 30,
    width: "50%",
  },
  timer: {
    marginBottom: 15,
    fontSize: 16,
    color: "#888",
  },
  expiredText: {
    fontSize: 16,
    color: "red",
    marginVertical: 20,
  },

  code: {
    fontSize: 40,
    fontWeight: "bold",
    letterSpacing: 8,
    marginVertical: 20,
  },
});
