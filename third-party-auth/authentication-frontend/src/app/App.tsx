import {
    CircularProgress,
    Container,
    makeStyles,
} from "@material-ui/core";
import React, { useEffect } from "react";
import { useSelector } from "react-redux";
import { Switch, useHistory } from "react-router-dom";
import Center from "../components/Center";
import { AuthActions, AuthSelectors } from "../features/auth/authSlice";
import SignInPage from "../features/auth/SignInPage";
import SignUpPage from "../features/auth/SignUpPage";
import Header from "../features/header/Header";
import UserList from "../features/user/UserList";
import { useAppDispatch } from "./hooks";
import Route from "./Route";
import Routes from "./Routes";

const useStyles = makeStyles(
    (theme) => ({
        container: {
            paddingTop: theme.spacing(2),
        },
    }),
    { name: "App" }
);

const App = () => {
    const classes = useStyles();
    const dispatch = useAppDispatch();
    const history = useHistory();
    const status = useSelector(AuthSelectors.selectStatus);

    useEffect(() => {
        dispatch(AuthActions.authenticateFromToken());
    }, [dispatch, history]);

    return (
        <div>
            <Header />
            <Container className={classes.container}>
                {status === "initializing" ? (
                    <Center>
                        <CircularProgress />
                    </Center>
                ) : (
                    <Switch>
                        <Route secure exact path={Routes.HOME}>
                            <UserList />
                        </Route>
                        <Route exact path={Routes.SIGN_IN}>
                            <SignInPage />
                        </Route>
                        <Route exact path={Routes.SIGN_UP}>
                            <SignUpPage />
                        </Route>
                    </Switch>
                )}
            </Container>
        </div>
    );
};

export default App;
