import { CameraView, useCameraPermissions } from "expo-camera";
import { StyleSheet, Button, TouchableOpacity, View, Text } from "react-native";
import { useState } from "react";

import { useAuth } from "@/context/AuthContext";
import {
  ChallengePayload,
  sendProof,
  validateChallengePayload,
} from "@/services/zkpService";

import { ThemedText } from "@/components/themed-text";
import { ThemedView } from "@/components/themed-view";

export default function Scanner() {
  const [permission, requestPermission] = useCameraPermissions();
  const [scanned, setScanned] = useState(false);
  const [scanData, setScanData] = useState<ChallengePayload>(
    {} as ChallengePayload,
  );

  const { getUserData } = useAuth();

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

  const handleScan = ({ data }: any) => {
    // TO DO: REMEMBER TO VERIFY SCAN DATA
    setScanned(true);
    let challenge = validateChallengePayload(data);

    console.log("QR code:", data);
    if (challenge) {
      setScanData(challenge);
      console.log("Valid challenge");
    }
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
      </ThemedView>
    );
  }

  const handleYes = async () => {
    // TO DO: REMEMBER TO VERIFY SCAN DATA
    console.log(scanData.callbackUrl);
    let userData = await getUserData("Send proof to 3rd party");
    if (userData) {
      await sendProof(userData, scanData);
    }
    setScanned(false);
    setScanData({} as ChallengePayload);
  };

  const handleNo = () => {
    setScanned(false);
    setScanData({} as ChallengePayload);
  };

  return (
    <View style={styles.container}>
      <Text style={styles.text}>Would you like to verify your age with:</Text>

      <Text style={styles.text}>{scanData.verifierName}</Text>

      <Text style={styles.text}>{scanData.callbackUrl}</Text>

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
