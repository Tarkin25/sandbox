import { StackHeaderProps } from '@react-navigation/stack';
import React from 'react';
import { Appbar } from 'react-native-paper';

const StackHeader = (props: StackHeaderProps) => {

    const { navigation: { canGoBack, goBack }, options: { title }, route: { name }} = props;

    return (
        <Appbar.Header>
            {canGoBack() && <Appbar.BackAction onPress={goBack} />}
            <Appbar.Content title={title || name} />
        </Appbar.Header>
    )
}

export default StackHeader
