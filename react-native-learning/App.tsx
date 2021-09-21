import { NavigationContainer } from "@react-navigation/native";
import {
  createStackNavigator,
} from "@react-navigation/stack";
import React from "react";
import { Provider as PaperProvider } from "react-native-paper";
import NavigationBar from "./components/NavigationBar";
import DetailScreen from "./features/detail/DetailScreen";
import HomeScreen from "./features/home/HomeScreen";

export type RouteParams = {
  Home: undefined;
  Details: undefined;
};

const Stack = createStackNavigator<RouteParams>();

const App = () => (
  <PaperProvider>
    <NavigationContainer>
      <Stack.Navigator
        initialRouteName="Home"
        screenOptions={{
          header: NavigationBar,
        }}
      >
          <Stack.Screen name="Home" component={HomeScreen} />
          <Stack.Screen name="Details" component={DetailScreen} />
      </Stack.Navigator>
    </NavigationContainer>
  </PaperProvider>
);

export default App;
