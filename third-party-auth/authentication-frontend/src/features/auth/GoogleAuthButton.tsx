import { makeStyles } from '@material-ui/core';
import React from 'react'
import GoogleLogin, { GoogleLoginResponse, GoogleLoginResponseOffline } from 'react-google-login';
import config from "../../app/config.json";
import { useAppDispatch } from '../../app/hooks';
import { AuthActions } from './authSlice';

export type GoogleAuthButtonProps = {
    action: "signIn" | "signUp";
}

const useStyles = makeStyles(theme => ({
    root: {
        display: "flex",
        flexDirection: "column"
    }
}), { name: "GoogleAuthButton" });

const isLoginResponse = (response: GoogleLoginResponse | GoogleLoginResponseOffline): response is GoogleLoginResponse => 
// @ts-ignore
typeof response.googleId === "string";

const GoogleAuthButton = (props: GoogleAuthButtonProps) => {

    const { action } = props;
    const dispatch = useAppDispatch();
    const classes = useStyles();

    const actionCreator = action === "signIn" ? AuthActions.googleSignIn : AuthActions.googleSignUp;
    const buttonText = action === "signIn" ? "Sign in with Google" : "Sign up with Google";

    const handleSuccess = (response: GoogleLoginResponse | GoogleLoginResponseOffline) => {
        if(isLoginResponse(response)) {
            dispatch(actionCreator(response.getAuthResponse().id_token));
        }
    }

    return (
        <div className={classes.root}>
            <GoogleLogin
                clientId={config.googleClientId}
                onSuccess={handleSuccess}
                buttonText={buttonText}
            />
        </div>
    )
}

export default GoogleAuthButton
