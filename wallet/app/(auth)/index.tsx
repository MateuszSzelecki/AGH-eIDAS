import { Redirect } from 'expo-router';
import { useAuth } from '../../context/AuthContext';
import { useEffect, useState } from 'react';
import { ActivityIndicator, View } from 'react-native';


export default function Index() {
  const { user, loading, biometricsCheck } = useAuth();
  const [authenticated, setAuthenticated] = useState<boolean>(false);

  useEffect(() => {
    const authenticate = async (): Promise<void> => {
      if (!user) {
        return;
      }
      else{

      const result = await biometricsCheck('Unlock App');
      setAuthenticated(result);
      return;
      }
    };

    authenticate();
  }, [user]);


    console.log(user)

  if (loading) {
    return (
      <View>
        <ActivityIndicator />
      </View>
    );
  }

  if (!user) {

    return <Redirect href="/login" />;
  }


  if (!authenticated) {
    return (
      <View>
        <ActivityIndicator />
      </View>
    );
  }


  return <Redirect href="/home" />;
}
