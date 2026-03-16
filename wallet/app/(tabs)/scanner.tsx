import { CameraView, useCameraPermissions } from 'expo-camera';
import { StyleSheet, Button } from 'react-native';
import { useState } from 'react';


import { ThemedText } from '@/components/themed-text';
import { ThemedView } from '@/components/themed-view';

export default function Scanner() {
const [permission, requestPermission] = useCameraPermissions();
  const [scanned, setScanned] = useState(false);

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
    setScanned(true);
    console.log("QR code:", data);
  };

  return (
    <ThemedView style={styles.container}>
      <CameraView
        style={StyleSheet.absoluteFillObject}
        barcodeScannerSettings={{
          barcodeTypes: ["qr"],
        }}
        onBarcodeScanned={scanned ? undefined : handleScan}
      />

      {scanned && (
        <Button title="Scan again" onPress={() => setScanned(false)} />
      )}
    </ThemedView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  center: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
  },
});
