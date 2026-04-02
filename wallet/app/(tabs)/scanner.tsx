import { CameraView, useCameraPermissions } from "expo-camera";
import { StyleSheet, Button, TouchableOpacity, View, Text } from "react-native";
import { useState } from "react";

import { useAuth } from "@/context/AuthContext";
import { sendProof } from "@/services/zkpService";

import { ThemedText } from "@/components/themed-text";
import { ThemedView } from "@/components/themed-view";

export default function Scanner() {
  const [permission, requestPermission] = useCameraPermissions();
  const [scanned, setScanned] = useState(false);
  const [scanData, setScanData] = useState("");

  const { user, biometricsCheck, getUserData } = useAuth();

  if (!permission) {
    return <ThemedView />;
  }

  if (!permission.granted) {
    return (
      <ThemedView style={styles.center}>
        <ThemedText>Camera permission is required</ThemedText>
        <Button title="Allow Camera" onPress={requestPermission} />
      </ThemedView>
    );
  }

  const handleScan = ({ data }) => {
    // TO DO: REMEMBER TO VERIFY SCAN DATA
    setScanned(true);
    setScanData(data);
    console.log("QR code:", data);
  };

  if (!scanned) {
    return (
      <ThemedView style={styles.container}>
        <CameraView
          style={StyleSheet.absoluteFillObject}
          barcodeScannerSettings={{
            barcodeTypes: ["qr"],
          }}
          onBarcodeScanned={scanned ? undefined : handleScan}
        />

        <Button title="test scan" onPress={() => handleScan("test")} />
      </ThemedView>
    );
  }

  const handleYes = async () => {
    // TO DO: REMEMBER TO VERIFY SCAN DATA
    await sendProof(await getUserData("Send proof to 3rd party"), scanData);
    setScanned(false);
    setScanData("");
  };

  const handleNo = () => {
    setScanned(false);
    setScanData("");
  };

  return (
    <View style={styles.container}>
      <Text style={styles.text}>Would you like to verify your age with:</Text>

      <Text style={styles.text}>{scanData}</Text>

      <View style={styles.buttons}>
        <TouchableOpacity style={styles.yesButton} onPress={handleYes}>
          <Text style={styles.buttonText}>Yes</Text>
        </TouchableOpacity>

        <TouchableOpacity style={styles.noButton} onPress={handleNo}>
          <Text style={styles.buttonText}>No</Text>
        </TouchableOpacity>
      </View>
    </View>
  );
}
const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    padding: 24,
  },
  text: {
    fontSize: 22,
    textAlign: "center",
    marginBottom: 32,
  },
  buttons: {
    flexDirection: "row",
    gap: 16,
  },
  yesButton: {
    backgroundColor: "#4CAF50",
    paddingVertical: 14,
    paddingHorizontal: 28,
    borderRadius: 8,
  },
  noButton: {
    backgroundColor: "#F44336",
    paddingVertical: 14,
    paddingHorizontal: 28,
    borderRadius: 8,
  },
  buttonText: {
    color: "white",
    fontSize: 16,
    fontWeight: "600",
  },
  center: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
  },
});
