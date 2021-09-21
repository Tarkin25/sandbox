import { StyleSheet } from "react-native";
import { useTheme } from "react-native-paper"

export type StyleCreator<T extends StyleSheet.NamedStyles<T>> = (theme: ReactNativePaper.Theme) => StyleSheet.NamedStyles<T>;

export const createStyles = <T extends StyleSheet.NamedStyles<T>>(styleCreator: StyleCreator<T>) => () => {
    const theme = useTheme();

    return StyleSheet.create(styleCreator(theme));
}