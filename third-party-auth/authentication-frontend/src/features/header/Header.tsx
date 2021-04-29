import { AppBar, Button, makeStyles, Toolbar, Typography } from '@material-ui/core'
import React, { Fragment } from 'react'
import { useSelector } from 'react-redux';
import { useAppDispatch } from '../../app/hooks';
import { AuthActions, AuthSelectors } from '../auth/authSlice';

const useStyles = makeStyles(theme => ({
    title: {
        flexGrow: 1
    }
}), { name: "Header" });

const Header = () => {

    const dispatch = useAppDispatch();
    const user = useSelector(AuthSelectors.selectUser);
    const classes = useStyles();

    const signOut = () => {
        dispatch(AuthActions.signOut());
    }

    return (
        <AppBar position="static">
            <Toolbar>
                <Typography variant="h4" className={classes.title}>
                    Authentication Demo
                </Typography>
                {user && (
                    <Fragment>
                        <Typography variant="h6">
                            {user.name}
                        </Typography>
                        <Button onClick={signOut} >Sign Out</Button>
                    </Fragment>
                )}
            </Toolbar>
        </AppBar>
    )
}

export default Header
