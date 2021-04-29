import { List, ListItem, ListItemText, Typography } from '@material-ui/core';
import React, { useEffect } from 'react'
import { useAppDispatch, useAppSelector } from '../../app/hooks'
import { UserActions } from './userSlice';

const UserList = () => {

    const dispatch = useAppDispatch();
    const users = useAppSelector(state => state.users);

    useEffect(() => {
        dispatch(UserActions.fetchAll());
    }, [dispatch]);

    return (
        <div>
            <Typography variant="h6">Registered Users</Typography>
            
            <List>
                {users.map(user => (
                    <ListItem key={user.id}>
                        <ListItemText primary={user.name} secondary={user.email} />
                    </ListItem>
                ))}
            </List>
        </div>
    )
}

export default UserList
