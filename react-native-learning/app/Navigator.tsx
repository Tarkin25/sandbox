import { createDrawerNavigator } from "@react-navigation/drawer";
import { createStackNavigator } from "@react-navigation/stack";
import React, { Fragment } from "react";
import DrawerContent from "../components/DrawerContent";
import DrawerNavigationBar from "../components/DrawerNavigationBar";
import StackNavigationBar from "../components/StackNavigationBar";
import LoginScreen from "../features/authentication/LoginScreen";
import DetailScreen from "../features/detail/DetailScreen";
import HomeScreen from "../features/home/HomeScreen";
import ProfileScreen from "../features/profile/ProfileScreen";
import { useAppSelector } from "./store";

export type StackParams = {
  Login: undefined;
};

export type DrawerParams = {
  Home: undefined;
  Profile: undefined;
};

const Stack = createStackNavigator<StackParams>();
const Drawer = createDrawerNavigator<DrawerParams>();

const Navigator = () => {
  const status = useAppSelector((state) => state.authentication.status);

  if (status === "authenticated") {
    return (
      <Drawer.Navigator
        screenOptions={{
          header: DrawerNavigationBar,
        }}
        drawerContent={(props) => <DrawerContent {...props} />}
      >
        <Drawer.Screen name="Home" component={HomeScreen} />
        <Drawer.Screen name="Profile" component={ProfileScreen} />
      </Drawer.Navigator>
    );
  } else {
    return (
      <Stack.Navigator
        screenOptions={{
          header: StackNavigationBar,
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
