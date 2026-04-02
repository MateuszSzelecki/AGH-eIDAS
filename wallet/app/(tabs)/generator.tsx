import { StyleSheet, TouchableOpacity, View } from "react-native";
import { ThemedText } from "@/components/themed-text";
import { ThemedView } from "@/components/themed-view";

import { useRouter } from "expo-router";

export default function Home() {
  const router = useRouter();
  return (
    <ThemedView style={styles.container}>
      <View style={styles.buttonStack}>
        <TouchableOpacity
          style={styles.secondaryButton}
          onPress={() => router.navigate("../generate/gen-qr")}
        >
          <ThemedText>Generate QR Code</ThemedText>
        </TouchableOpacity>

        <TouchableOpacity style={styles.secondaryButton}>
          <ThemedText onPress={() => router.navigate("../generate/gen-code")}>
            Generate Numeric Code
          </ThemedText>
        </TouchableOpacity>
      </View>
    </ThemedView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "flex-start",
    padding: 20,
  },
  buttonText: {
    color: "#fff",
    fontWeight: "bold",
    fontSize: 18,
  },
  buttonStack: {
    width: "100%",
    gap: 15,
  },
  secondaryButton: {
    borderWidth: 1,
    borderColor: "#ccc",
    padding: 15,
    borderRadius: 8,
    alignItems: "center",
  },
});
