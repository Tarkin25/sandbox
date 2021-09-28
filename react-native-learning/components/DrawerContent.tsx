import { DrawerContentComponentProps } from "@react-navigation/drawer";
import React from "react";
import { SafeAreaView, ScrollView } from "react-native";
import { Drawer } from "react-native-paper";
import { createStyles } from "../utils/createStyles";

const DrawerContent = (props: DrawerContentComponentProps) => {

  const descriptors = Object.values(props.descriptors);
  const styles = useStyles();
    const { state: { index, routes } } = props;

  return (
    <ScrollView style={styles.root}>
      <SafeAreaView>
          {descriptors.map((descriptor) => {
            const {
              route: { name },
              options: { title },
            } = descriptor;

            const isActive = routes[index].name === name;

            return <Drawer.Item key={name} active={isActive} label={title || name} onPress={() => props.navigation.navigate(name)} />;
          })}
      </SafeAreaView>
    </ScrollView>
  );
};

const useStyles = createStyles(theme => ({
    root: {
        backgroundColor: theme.colors.surface
    }
}))

export default DrawerContent;
