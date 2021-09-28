import React, { ReactNode } from 'react'
import { StyleProp, StyleSheet, View, ViewStyle } from 'react-native';
import { useTheme } from 'react-native-paper';
import { createStyles } from '../utils/createStyles';

export interface ScreenWrapperProps {
    children?: ReactNode;
    style?: StyleProp<ViewStyle>;
}

const ScreenWrapper = (props: ScreenWrapperProps) => {

    const { children, style } = props;
    const styles = useStyles();

    return (
        <View style={[style, styles.container]}>
            {children}
        </View>
    )
}

const useStyles = createStyles(theme => ({
    container: {
        flex: 1,
        backgroundColor: theme.colors.background,
        padding: 24
    }
}))

export default ScreenWrapper
