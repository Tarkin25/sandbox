import { StackScreenProps } from "@react-navigation/stack";
import React, { ChangeEvent, useState } from "react";
import { Button, Text, TextInput } from "react-native-paper";
import { StackParams } from "../../app/Navigator";
import { useAppDispatch, useAppSelector } from "../../app/store";
import ScreenWrapper from "../../components/ScreenWrapper";
import { createStyles } from "../../utils/createStyles";
import { AuthenticationActions } from "./authenticationSlice";

export interface LoginScreenProps
  extends StackScreenProps<StackParams, "Login"> {}

const LoginScreen = (props: LoginScreenProps) => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const dispatch = useAppDispatch();
  const styles = useStyles();
  const status = useAppSelector((state) => state.authentication.status);

  const handleSignIn = () => {
    dispatch(AuthenticationActions.login({ username, password }));
  };

  return (
    <ScreenWrapper>
      <TextInput
        label="Username"
        value={username}
        onChangeText={setUsername}
        style={styles.marginBottom}
      />
      <TextInput
        label="Password"
        value={password}
        onChangeText={setPassword}
        secureTextEntry
        style={styles.marginBottom}
      />
      {status === "authentication-failed" && (
        <Text style={[styles.marginBottom, styles.textRed]}>
          Invalid username or password
        </Text>
      )}
      <Button
        mode="outlined"
        onPress={handleSignIn}
        loading={status === "pending"}
      >
        Sign In
      </Button>
    </ScreenWrapper>
  );
};

const useStyles = createStyles((theme) => ({
  marginBottom: {
    marginBottom: 24,
  },
  textRed: {
    color: theme.colors.error,
  },
}));

export default LoginScreen;
