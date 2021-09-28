import { DrawerHeaderProps } from '@react-navigation/drawer';
import React from 'react';
import { Appbar } from 'react-native-paper';

const DrawerNavigationBar = (props: DrawerHeaderProps) => {

    const { navigation, route: { name }, options: { title }} = props;

    return (
        <Appbar.Header>
            <Appbar.Action icon="menu" onPress={navigation.toggleDrawer} />
            <Appbar.Content title={title || name} />
        </Appbar.Header>
    )
}

export default DrawerNavigationBar
