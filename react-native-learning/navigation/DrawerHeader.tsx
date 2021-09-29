import { DrawerHeaderProps } from '@react-navigation/drawer'
import React from 'react'
import { Appbar } from 'react-native-paper'

const DrawerHeader = (props: DrawerHeaderProps) => {

    const { navigation: { toggleDrawer }, options: { title }, route: { name }} = props;

    return (
        <Appbar.Header>
            <Appbar.Action icon="menu" onPress={toggleDrawer} />
            <Appbar.Content title={title || name} />
        </Appbar.Header>
    )
}

export default DrawerHeader
