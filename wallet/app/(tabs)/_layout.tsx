import { Tabs } from "expo-router";

import MaterialIcons from '@expo/vector-icons/MaterialIcons';

import { Colors } from '@/constants/theme';
import { useColorScheme } from '@/hooks/use-color-scheme';

export default function TabsLayout() {

  const colorScheme = useColorScheme();
  return (
    <Tabs
      screenOptions={{
        headerShown: false,
        tabBarActiveTintColor: Colors[colorScheme ?? 'light'].tint,
      }}
    >
      <Tabs.Screen
        name="home"
        options={{
          title: "ID",
          tabBarIcon: ({color}) => <MaterialIcons size={28} color={color} name={"person"} />,
        }}
      />

      <Tabs.Screen
        name="generator"
        options={{
          title: "Generate",
          tabBarIcon: ({color}) => <MaterialIcons size={28} color={color} name={"qr-code"} />,
        }}
      />
      <Tabs.Screen
        name="scanner"
        options={{
          title: "Scanner",
          tabBarIcon: ({color}) => <MaterialIcons size={28} color={color} name={"qr-code-scanner"} />,
        }}
      />

      <Tabs.Screen
        name="settings"
        options={{
          title: "Settings",
          tabBarIcon: ({color}) => <MaterialIcons size={28} color={color} name={"settings"} />,
        }}
      />
    </Tabs>
  );
}
