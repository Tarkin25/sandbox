import { StackScreenProps } from '@react-navigation/stack'
import React from 'react'
import { StyleSheet } from 'react-native'
import { Text } from 'react-native-paper'
import { RouteParams } from '../../App'
import ScreenWrapper from '../../components/ScreenWrapper'
import { createStyles } from '../../utils/createStyles'

export interface DetailScreenProps extends StackScreenProps<RouteParams, 'Details'> {}

const DetailScreen = (props: DetailScreenProps) => {

    const styles = useStyles()

    return (
        <ScreenWrapper style={styles.root}>
            <Text style={styles.text} >This is the details screen.</Text>
        </ScreenWrapper>
    )
}

const useStyles = createStyles(theme => ({
    root: {
        justifyContent: 'center',
        alignItems: 'center'
    },
    text: {
        fontSize: 24
    }
}))

export default DetailScreen
