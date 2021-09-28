import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";

export interface User {
    username: string;
}

export type AuthenticationStatus = 'unauthenticated' | 'pending' | 'authenticated' | 'authentication-failed'

interface AuthenticationState {
    status: AuthenticationStatus;
    user: User | undefined;
}

const initialState: AuthenticationState = {
    status: 'authenticated',
    user: {
        username: 'Severin'
    }
};

// ACTIONS

export interface Login {
    username: string;
    password: string;
}

const login = createAsyncThunk(
    'authentication/login',
    (login: Login): Promise<User> => new Promise((resolve, reject) => {
        setTimeout(() => {
            if (login.password === '12345') {
                resolve({username: login.username});
            } else {
                reject('invalid username or password');
            }
        }, 500);
    })
)

const logout = createAsyncThunk(
    'authentication/logout',
    async () => {}
)

const slice = createSlice({
    initialState,
    name: 'authentication',
    reducers: {

    },
    extraReducers: builder => builder
    .addCase(login.pending, (state) => {
        state.status = 'pending';
    })
    .addCase(login.fulfilled, (state, action) => {
        state.status = 'authenticated';
        state.user = action.payload;
    })
    .addCase(login.rejected, (state) => {
        state.status = 'authentication-failed';
    })
    .addCase(logout.fulfilled, (state) => {
        state.status = 'unauthenticated';
        state.user = undefined;
    })
});

export const AuthenticationActions = {
    login,
    logout
}

export const authenticationReducer = slice.reducer;