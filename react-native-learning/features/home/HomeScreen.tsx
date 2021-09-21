import { StackScreenProps } from '@react-navigation/stack'
import React from 'react'
import { StyleSheet } from 'react-native'
import { Button, Text } from 'react-native-paper'
import { RouteParams } from '../../App'
import ScreenWrapper from '../../components/ScreenWrapper'

export interface HomeScreenProps extends StackScreenProps<RouteParams, 'Home'> {

}

const HomeScreen = (props: HomeScreenProps) => {

    const {navigation} = props;

    return (
        <ScreenWrapper style={styles.root}>
            <Text>This is the home screen.</Text>
            <Button mode="contained" onPress={() => navigation.navigate('Details')}>Go to details screen</Button>
        </ScreenWrapper>
    )
}

export default HomeScreen

const styles = StyleSheet.create({
    root: {
        alignItems: 'center',
        justifyContent: 'center'
    }
})
