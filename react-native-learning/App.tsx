import 'react-native-gesture-handler';
import { NavigationContainer } from "@react-navigation/native";
import React from "react";
import { Provider as PaperProvider } from "react-native-paper";
import { Provider as ReduxProvider } from "react-redux";
import Navigator from "./app/Navigator";
import { store } from "./app/store";

const App = () => (
  <ReduxProvider store={store}>
    <PaperProvider>
      <NavigationContainer>
        <Navigator />
      </NavigationContainer>
    </PaperProvider>
  </ReduxProvider>
);

export default App;
