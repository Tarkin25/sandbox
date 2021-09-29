import { StackHeaderProps } from '@react-navigation/stack'
import React, { useState } from 'react'
import { Appbar, Divider, Menu } from 'react-native-paper'
import { MainStackParams } from './MainStackNavigator';

const MainHeader = (props: StackHeaderProps) => {

    const { navigation, options: { title }, route: { name }} = props;

    const [menuOpen, setMenuOpen] = useState(false);

    const openMenu = () => setMenuOpen(true);
    const closeMenu = () => setMenuOpen(false);

    const navigate = (screen: keyof MainStackParams) => () => {
        closeMenu();
        navigation.navigate(screen);
    }

    return (
        <Appbar.Header>
            <Appbar.Content title={title || name} />
            <Menu
                visible={menuOpen}
                onDismiss={closeMenu}
                anchor={<Appbar.Action icon="dots-vertical" color="white" onPress={openMenu} />}
            >
                <Menu.Item title="Profile" onPress={navigate('Profile')} />
                <Divider />
                <Menu.Item title="Settings" />
            </Menu>
        </Appbar.Header>
    )
}

export default MainHeader
