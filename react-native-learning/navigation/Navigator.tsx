import { createStackNavigator } from "@react-navigation/stack";
import React from "react";
import { useAppSelector } from "../app/store";
import LoginScreen from "../features/authentication/LoginScreen";
import MainStackNavigator from "./MainStackNavigator";
import StackHeader from "./StackHeader";

export type StackParams = {
  Login: undefined;
};

const Stack = createStackNavigator<StackParams>();

const Navigator = () => {
  const status = useAppSelector((state) => state.authentication.status);

  if (status === "authenticated") {
    return (
      <MainStackNavigator />
    );
  } else {
    return (
      <Stack.Navigator
        screenOptions={{
          header: StackHeader,
        }}
      >
        <Stack.Screen
          name="Login"
          options={{ title: "Sign In" }}
          component={LoginScreen}
        />
      </Stack.Navigator>
    );
  }
};

export default Navigator;
