import { createDrawerNavigator } from '@react-navigation/drawer'
import { StackScreenProps } from '@react-navigation/stack'
import React from 'react'
import DrawerContent from './DrawerContent'
import HomeScreen from '../features/home/HomeScreen'
import DrawerHeader from './DrawerHeader'
import { MainStackParams } from './MainStackNavigator'

type HomeParams = {
    Home: undefined;
}

const Drawer = createDrawerNavigator<HomeParams>();

const HomeNavigator = (props: StackScreenProps<MainStackParams, 'HomeNavigator'>) => {
    return (
        <Drawer.Navigator drawerContent={props => <DrawerContent {...props} />} screenOptions={{header: DrawerHeader}}>
            <Drawer.Screen name="Home" component={HomeScreen} />
        </Drawer.Navigator>
    )
}

export default HomeNavigator
