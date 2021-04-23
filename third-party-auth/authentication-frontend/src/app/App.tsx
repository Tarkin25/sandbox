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
import api from "./api";

const clientId =
    "123219520317-hp1o583h2ceh28uln9o62a9lscq67ufn.apps.googleusercontent.com";

const isLoginResponse = (result: any): result is GoogleLoginResponse =>
    typeof result.googleId === "string";

const App = () => {
    const handleSuccess = (
        result: GoogleLoginResponse | GoogleLoginResponseOffline
    ) => {
        if (isLoginResponse(result)) {
            console.log(result);
            
            api.post("/sign-in/google", undefined, {params: {googleIdToken: result.getAuthResponse().id_token}})
            .then(res => {
              console.log(res.data);
            })
        }
    };

    const { signIn } = useGoogleLogin({
        clientId,
        onSuccess: handleSuccess,
    });

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
                        onClick={signIn}
                    >
                        Sign in with Google
                    </Button>
                </Box>
            </Container>
        </div>
    );
};

export default App;
