import { DrawerContentComponentProps } from "@react-navigation/drawer";
import React from "react";
import { SafeAreaView, ScrollView } from "react-native";
import { Drawer } from "react-native-paper";
import { useAppDispatch } from "../app/store";
import { AuthenticationActions } from "../features/authentication/authenticationSlice";
import { createStyles } from "../utils/createStyles";

const routes = [
    { name: "HomeNavigator", title: "Home" },
    { name: "Profile", title: "Profile" },
];

const DrawerContent = (props: DrawerContentComponentProps) => {
    const styles = useStyles();
    const {
        state: { index },
        navigation,
    } = props;
    const dispatch = useAppDispatch();

    const navigate = (screen: string) => () => {
        navigation.closeDrawer();
        navigation.navigate(screen);
    }

    return (
        <ScrollView style={styles.root}>
            <SafeAreaView>
                <Drawer.Section>
                    {routes.map(route => (
                        <Drawer.Item key={route.name} onPress={navigate(route.name)} label={route.title} />
                    ))}
                </Drawer.Section>
                <Drawer.Section>
                    <Drawer.Item
                        label="Sign Out"
                        onPress={() => dispatch(AuthenticationActions.logout())}
                    />
                </Drawer.Section>
            </SafeAreaView>
        </ScrollView>
    );
};

const useStyles = createStyles((theme) => ({
    root: {
        backgroundColor: theme.colors.surface,
    },
}));

export default DrawerContent;
