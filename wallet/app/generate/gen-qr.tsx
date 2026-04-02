import { View, Text, ActivityIndicator, Button } from "react-native";
import { useAuth } from "@/context/AuthContext";
import { useEffect, useState, useRef } from "react";
import QRCode from "react-native-qrcode-svg";
import { router } from "expo-router";
import { generateProof } from "@/services/zkpService";

import { StyleSheet } from "react-native";
import { ThemedText } from "@/components/themed-text";
import { ThemedView } from "@/components/themed-view";

type QRResponse = {
  qrValue: string;
};

export default function QRScreen() {
  const { user, biometricsCheck, getUserData } = useAuth();
  const [loading, setLoading] = useState<boolean>(true);
  const [qrData, setQrData] = useState<string | null>(null);
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
          setQrData(null);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  };

  const authenticateAndGenerate = async () => {
    setLoading(true);

    const data = await generateProof(await getUserData("Generate QR code"));

    if (data) {
      setAuthenticated(true);
      setQrData(data);
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
        Scan this code to verify your age:
      </ThemedText>

      {!expired && qrData && (
        <Text style={styles.timer}>Expires in: {timeLeft}s</Text>
      )}

      {qrData && !expired ? (
        <QRCode value={qrData} size={300} />
      ) : (
        <Text style={styles.expiredText}>QR Code expired</Text>
      )}

      <View style={styles.buttonContainer}>
        {expired && (
          <Button title="Regenerate QR Code" onPress={handleRegenerate} />
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
});
