import { createStackNavigator } from "@react-navigation/stack";
import React from "react";
import DetailScreen from "../features/detail/DetailScreen";
import ProfileScreen from "../features/profile/ProfileScreen";
import HomeNavigator from "./HomeNavigator";
import StackHeader from "./StackHeader";

export type MainStackParams = {
    HomeNavigator: undefined;
    Profile: undefined;
    Details: undefined;
};

const Stack = createStackNavigator<MainStackParams>();

const MainStackNavigator = () => {
    return (
        <Stack.Navigator screenOptions={{header: StackHeader}}>
            <Stack.Screen
                name="HomeNavigator"
                component={HomeNavigator}
                options={{headerShown: false, title: 'Home'}}
            />
            <Stack.Screen name="Profile" component={ProfileScreen} />
            <Stack.Screen name="Details" component={DetailScreen} />
        </Stack.Navigator>
    );
};

export default MainStackNavigator;
