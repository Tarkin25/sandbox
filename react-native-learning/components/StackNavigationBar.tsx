import { StackHeaderProps } from '@react-navigation/stack'
import React from 'react'
import { Appbar } from 'react-native-paper'

const StackNavigationBar = (props: StackHeaderProps) => {

    const { navigation, back, route: { name }, options: { title } } = props;

    return (
        <Appbar.Header>
            {back && <Appbar.BackAction onPress={() => navigation.goBack()} /> }
            <Appbar.Content title={title || name} />
        </Appbar.Header>
    )
}

export default StackNavigationBar
