import { createStackNavigator } from "@react-navigation/stack";
import React from "react";
import StackNavigationBar from "../components/StackNavigationBar";
import DetailScreen from "../features/detail/DetailScreen";
import HomeScreen from "../features/home/HomeScreen";
import ProfileScreen from "../features/profile/ProfileScreen";
import MainHeader from "./MainHeader";

export type MainStackParams = {
    Home: undefined;
    Profile: undefined;
    Details: undefined;
};

const Stack = createStackNavigator<MainStackParams>();

const MainStackNavigator = () => {
    return (
        <Stack.Navigator screenOptions={{header: StackNavigationBar}}>
            <Stack.Screen
                name="Home"
                component={HomeScreen}
                options={{ header: MainHeader, title: "Home" }}
            />
            <Stack.Screen name="Profile" component={ProfileScreen} />
            <Stack.Screen name="Details" component={DetailScreen} />
        </Stack.Navigator>
    );
};

export default MainStackNavigator;
