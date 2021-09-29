import { DrawerScreenProps } from "@react-navigation/drawer";
import React from "react";
import { Button, Text } from "react-native-paper";
import ScreenWrapper from "../../components/ScreenWrapper";
import { MainStackParams } from "../../navigation/MainStackNavigator";
import { createStyles } from "../../utils/createStyles";

export interface HomeScreenProps
    extends DrawerScreenProps<MainStackParams, "HomeNavigator"> {}

const HomeScreen = (props: HomeScreenProps) => {
    const { navigation } = props;
    const styles = useStyles();

    return (
        <ScreenWrapper style={styles.root}>
            <Text>This is the home screen.</Text>
            <Button onPress={() => navigation.navigate("Details")}>
                Go to Details
            </Button>
        </ScreenWrapper>
    );
};

export default HomeScreen;

const useStyles = createStyles(() => ({
    root: {
        justifyContent: "center",
        alignItems: "center",
    },
}));
