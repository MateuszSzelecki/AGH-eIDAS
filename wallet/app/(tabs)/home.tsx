import { StyleSheet, TouchableOpacity, View } from 'react-native';
import { ThemedText } from '@/components/themed-text';
import { ThemedView } from '@/components/themed-view';

import { useRouter } from 'expo-router';

export default function Home() {
const router = useRouter();
  return (
<ThemedView style={styles.container}>
      
      <TouchableOpacity style={styles.mainButton}>
        <ThemedText style={styles.buttonText}

                >My ID</ThemedText>
      </TouchableOpacity>

        <TouchableOpacity style={styles.secondaryButton}

    onPress={() => router.navigate('../documents/add')}
                >
          <ThemedText>Add Documents +</ThemedText>
        </TouchableOpacity>


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
  mainButton: {
    backgroundColor: '#007AFF',
    padding: 20,
    borderRadius: 12,
    width: '100%',
    alignItems: 'flex-start',
    height: 200,
    marginBottom: 30,
  },
  buttonText: {
    color: '#fff',
    fontWeight: 'bold',
    fontSize: 18,
  },
  secondaryButton: {
    borderWidth: 1,
    borderColor: '#ccc',
    padding: 15,
    borderRadius: 8,
    alignItems: 'flex-end',
  }
});
