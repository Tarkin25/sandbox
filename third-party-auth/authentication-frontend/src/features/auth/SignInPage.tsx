import { Card, CardContent, CardHeader, Grid } from "@material-ui/core";
import React from "react";
import Center from "../../components/Center";
import GoogleAuthButton from "./GoogleAuthButton";
import FacebookAuthButton from "./FacebookAuthButton";
import useRedirectOnAuthentication from "./useRedirectOnAuthentication";
import Link from "../../components/Link";
import Routes from "../../app/Routes";

const SignInPage = () => {

    useRedirectOnAuthentication();

    return (
        <Center>
            <Card>
                <CardHeader
                    title="Sign In"
                    titleTypographyProps={{ align: "center" }}
                />
                <CardContent>
                    <Grid container spacing={2} direction="column">
                        <Grid item>
                            <GoogleAuthButton action="signIn" />
                        </Grid>
                        <Grid item>
                            <FacebookAuthButton action="signIn" />
                        </Grid>
                        <Grid item>
                            <Link color="textPrimary" underline="always" to={Routes.SIGN_UP}>
                                Don't have an account? Sign up here.
                            </Link>
                        </Grid>
                    </Grid>
                </CardContent>
            </Card>
        </Center>
    );
};

export default SignInPage;
