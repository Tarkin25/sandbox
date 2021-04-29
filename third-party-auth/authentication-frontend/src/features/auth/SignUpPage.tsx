import React from 'react'
import Center from '../../components/Center';
import { Card, CardContent, CardHeader, Grid } from '@material-ui/core';
import GoogleAuthButton from './GoogleAuthButton';
import FacebookAuthButton from './FacebookAuthButton';
import useRedirectOnAuthentication from './useRedirectOnAuthentication';
import Link from '../../components/Link';
import Routes from '../../app/Routes';

const SignUpPage = () => {

    useRedirectOnAuthentication();

    return (
        <Center>
            <Card>
                <CardHeader title="Sign Up" titleTypographyProps={{align: "center"}} />
                <CardContent>
                    <Grid container spacing={2} direction="column">
                        <Grid item>
                            <GoogleAuthButton action="signUp" />
                        </Grid>
                        <Grid item>
                            <FacebookAuthButton action="signUp" />
                        </Grid>
                        <Grid item>
                            <Link color="textPrimary" underline="always" to={Routes.SIGN_IN} >
                                Already have an account? Sign in here.
                            </Link>
                        </Grid>
                    </Grid>
                </CardContent>
            </Card>
        </Center>
    )
}

export default SignUpPage
