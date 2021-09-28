import { DrawerContentComponentProps } from "@react-navigation/drawer";
import React from "react";
import { SafeAreaView, ScrollView } from "react-native";
import { Drawer } from "react-native-paper";
import { useAppDispatch } from "../app/store";
import { AuthenticationActions } from "../features/authentication/authenticationSlice";
import { createStyles } from "../utils/createStyles";

const DrawerContent = (props: DrawerContentComponentProps) => {
  const descriptors = Object.values(props.descriptors);
  const styles = useStyles();
  const {
    state: { index, routes },
  } = props;
  const dispatch = useAppDispatch();

  return (
    <ScrollView style={styles.root}>
      <SafeAreaView>
        <Drawer.Section>
          {descriptors.map((descriptor) => {
            const {
              route: { name },
              options: { title },
            } = descriptor;

            const isActive = routes[index].name === name;

            return (
              <Drawer.Item
                key={name}
                active={isActive}
                label={title || name}
                onPress={() => props.navigation.navigate(name)}
              />
            );
          })}
        </Drawer.Section>
        <Drawer.Section>
            <Drawer.Item label="Sign Out" onPress={() => dispatch(AuthenticationActions.logout())} />
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
