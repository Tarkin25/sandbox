import { DrawerScreenProps } from '@react-navigation/drawer'
import React from 'react'
import { Text } from 'react-native-paper'
import { DrawerParams } from '../../app/Navigator'
import ScreenWrapper from '../../components/ScreenWrapper'
import { createStyles } from '../../utils/createStyles'

export interface HomeScreenProps extends DrawerScreenProps<DrawerParams, 'Home'> {

}

const HomeScreen = (props: HomeScreenProps) => {

    const {navigation} = props;
    const styles = useStyles();

    return (
        <ScreenWrapper style={styles.root}>
            <Text>This is the home screen.</Text>
        </ScreenWrapper>
    )
}

export default HomeScreen

const useStyles = createStyles(() => ({
    root: {
        justifyContent: 'center',
        alignItems: 'center'
    }
}))
