import {
    AppBar,
    Box,
    Button,
    Container,
    Toolbar,
    Typography,
} from "@material-ui/core";
import React from "react";
import {
    GoogleLoginResponse,
    GoogleLoginResponseOffline,
    useGoogleLogin,
} from "react-google-login";
import FacebookLogin, { ReactFacebookFailureResponse, ReactFacebookLoginInfo } from "react-facebook-login";

const googleClientId =
    "123219520317-hp1o583h2ceh28uln9o62a9lscq67ufn.apps.googleusercontent.com";

const facebookAppId = "197602162004868";

const isLoginResponse = (result: GoogleLoginResponse | GoogleLoginResponseOffline): result is GoogleLoginResponse =>
  // @ts-ignore
    typeof result.googleId === "string";

const isFacebookLoginInfo = (info: ReactFacebookLoginInfo | ReactFacebookFailureResponse): info is ReactFacebookLoginInfo => 
 // @ts-ignore
    typeof info.id === "string";

const App = () => {
    const handleGoogleSignIn = (
        result: GoogleLoginResponse | GoogleLoginResponseOffline
    ) => {
        if (isLoginResponse(result)) {
            console.log("google sign-in", result.getAuthResponse().id_token);
        }
    };

    const handleGoogleSignUp = (
        result: GoogleLoginResponse | GoogleLoginResponseOffline
    ) => {
        if(isLoginResponse(result)) {
            console.log("google sign-up", result.getAuthResponse().id_token);
        }
    }

    const facebookSignIn = (userInfo: ReactFacebookLoginInfo | ReactFacebookFailureResponse) => {
        if(isFacebookLoginInfo(userInfo)) {
            console.log("facebook sign-in", userInfo.accessToken);
        }
    }

    const facebookSignUp = (userInfo: ReactFacebookLoginInfo | ReactFacebookFailureResponse) => {
        if(isFacebookLoginInfo(userInfo)) {
            console.log("facebook sign-up", userInfo.accessToken);
        }
    }

    const { signIn: googleSignIn } = useGoogleLogin({
        clientId: googleClientId,
        onSuccess: handleGoogleSignIn,
    });

    const {signIn: googleSignUp} = useGoogleLogin({
        clientId: googleClientId,
        onSuccess: handleGoogleSignUp
    })

    return (
        <div>
            <AppBar position="static">
                <Toolbar>
                    <Typography variant="h4">Authentication</Typography>
                </Toolbar>
            </AppBar>
            <Container>
                <Box m={4}>
                    <Button
                        color="primary"
                        variant="contained"
                        onClick={googleSignIn}
                    >
                        Sign in with Google
                    </Button>
                </Box>
                <Box m={4}>
                    <Button
                        color="primary"
                        variant="contained"
                        onClick={googleSignUp}
                    >
                        Sign up with Google
                    </Button>
                </Box>
                <Box m={4}>
                    <FacebookLogin
                        appId={facebookAppId}
                        callback={facebookSignIn}
                        textButton="Sign in with Facebook"
                    />
                </Box>
                <Box m={4}>
                    <FacebookLogin
                        appId={facebookAppId}
                        callback={facebookSignUp}
                        textButton="Sign up with Facebook"
                    />
                </Box>
            </Container>
        </div>
    );
};

export default App;
