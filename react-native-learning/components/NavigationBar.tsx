import { StackHeaderProps } from '@react-navigation/stack'
import React from 'react'
import { Appbar } from 'react-native-paper'

const NavigationBar = (props: StackHeaderProps) => {

    const { navigation, back, route: { name } } = props;

    return (
        <Appbar.Header>
            {back && <Appbar.BackAction onPress={() => navigation.goBack()} /> }
            <Appbar.Content title={name} />
        </Appbar.Header>
    )
}

export default NavigationBar
