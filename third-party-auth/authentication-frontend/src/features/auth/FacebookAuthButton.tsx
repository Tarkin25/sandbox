import { makeStyles } from '@material-ui/core'
import React from 'react'
import { useAppDispatch } from '../../app/hooks';
import config from "../../app/config.json";
import { AuthActions } from './authSlice';
import ReactFacebookLogin, { ReactFacebookFailureResponse, ReactFacebookLoginInfo } from 'react-facebook-login';
import FacebookIcon from "@material-ui/icons/Facebook";

export type FacebookAuthButtonProps = {
    action: "signIn" | "signUp";
}

const useStyles = makeStyles(theme => ({
    root: {
        display: "flex",
        flexDirection: "column",
        "& .kep-login-facebook.metro": {
            width: "100%",
            borderRadius: 2,
            fontSize: "14px",
            fontWeight: 500,
            textTransform: "none",
            display: "inline-flex",
            flexDirection: "row",
            alignItems: "center",
            padding: 0,
            paddingRight: 10,
            "& svg": {
                margin: "10px 10px 10px 10px"
            }
        }
    }
}), { name: "FacebookAuthButton" });

const isLoginInfo = (response: ReactFacebookLoginInfo | ReactFacebookFailureResponse): response is ReactFacebookLoginInfo => 
// @ts-ignore
typeof response.accessToken === "string";

const FacebookAuthButton = (props: FacebookAuthButtonProps) => {

    const { action } = props;
    const dispatch = useAppDispatch();
    const classes = useStyles();

    const actionCreator = action === "signIn" ? AuthActions.facebookSignIn : AuthActions.facebookSignUp;
    const buttonText = action === "signIn" ? "Sign in with Facebook" : "Sign up with Facebook";

    const callback = (response: ReactFacebookLoginInfo | ReactFacebookFailureResponse) => {
        if(isLoginInfo(response)) {
            dispatch(actionCreator(response.accessToken));
        }
    }

    return (
        <div className={classes.root}>
            <ReactFacebookLogin
                appId={config.facebookAppId}
                callback={callback}
                textButton={buttonText}
                icon={<FacebookIcon />}
            />
        </div>
    )
}

export default FacebookAuthButton
