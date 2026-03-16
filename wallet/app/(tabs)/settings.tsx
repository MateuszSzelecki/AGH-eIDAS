import { StyleSheet, TouchableOpacity, View } from 'react-native';
import { ThemedText } from '@/components/themed-text';
import { ThemedView } from '@/components/themed-view';

import { useRouter } from 'expo-router';

export default function Home() {
const router = useRouter();
  return (
<ThemedView style={styles.container}>
      
      <View style={styles.buttonStack}>
        <TouchableOpacity style={styles.secondaryButton}

                >
          <ThemedText>Setting #1</ThemedText>
        </TouchableOpacity>

        <TouchableOpacity style={styles.secondaryButton}
                >
          <ThemedText>Setting #2</ThemedText>
        </TouchableOpacity>

        <TouchableOpacity style={styles.secondaryButton}>

          <ThemedText>Setting #3</ThemedText>
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
    color: '#fff',
    fontWeight: 'bold',
    fontSize: 18,
  },
  buttonStack: {
    width: '100%',
    gap: 15,
  },
  secondaryButton: {
    borderWidth: 1,
    borderColor: '#ccc',
    padding: 15,
    borderRadius: 8,
    alignItems: 'center',
  }
});
