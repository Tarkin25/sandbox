import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import api from "../../app/api";
import { createSelectorCreator } from "../../app/selector";
import { User } from "../user/userSlice";

export type AuthStatus = "initializing" | "pending" | "unauthenticated" | "authentication-failed" | "authenticated";

export interface AuthState {
    status: AuthStatus;
    user: User | undefined;
}

interface AuthenticationResponse {
    authToken: string;
    user: User;
}

const initialState: AuthState = {
    status: "initializing",
    user: undefined
}

const AUTH_TOKEN_KEY = "authToken";

export const getAuthToken = () => localStorage.getItem(AUTH_TOKEN_KEY);

const setAuthToken = (token: string) => localStorage.setItem(AUTH_TOKEN_KEY, token);

const clearAuthToken = () => localStorage.removeItem(AUTH_TOKEN_KEY);

const authenticateFromToken = createAsyncThunk(
    "auth/authenticateFromToken",
    async () => {
        const res = await api.get<User>("/me");

        return res.data;
    }
)

const googleSignIn = createAsyncThunk(
    "auth/googleSignIn",
    async (googleIdToken: string) => {
        const res = await api.post<AuthenticationResponse>("/authentication/google/sign-in", undefined, {params: {googleIdToken}})

        setAuthToken(res.data.authToken);
        return res.data.user;
    }
)

const googleSignUp = createAsyncThunk(
    "auth/googleSignUp",
    async (googleIdToken: string) => {
        const res = await api.post<AuthenticationResponse>("/authentication/google/sign-up", undefined, {params: {googleIdToken}});

        setAuthToken(res.data.authToken);
        return res.data.user;
    }
)

const facebookSignIn = createAsyncThunk(
    "auth/facebookSignIn",
    async (accessToken: string) => {
        const res = await api.post<AuthenticationResponse>("/authentication/facebook/sign-in", undefined, {params: {accessToken}});

        setAuthToken(res.data.authToken);
        return res.data.user;
    }
)

const facebookSignUp = createAsyncThunk(
    "auth/facebookSignUp",
    async (accessToken: string) => {
        const res = await api.post<AuthenticationResponse>("/authentication/facebook/sign-up", undefined, {params: {accessToken}});

        setAuthToken(res.data.authToken);
        return res.data.user;
    }
)

const slice = createSlice({
    name: "auth",
    initialState,
    reducers: {
        signOut(state) {
            clearAuthToken();
            state.status = "unauthenticated";
            state.user = undefined;
        }
    },
    extraReducers: builder => builder
        .addCase(googleSignIn.pending, (state) => {
            state.status = "pending";
        })
        .addCase(googleSignIn.rejected, (state) => {
            state.status = "authentication-failed";
        })
        .addCase(googleSignIn.fulfilled, (state, action) => {
            state.status = "authenticated";
            state.user = action.payload;
        })
        .addCase(authenticateFromToken.pending, (state) => {
            state.status = "initializing";
        })
        .addCase(authenticateFromToken.rejected, (state) => {
            state.status = "unauthenticated";
        })
        .addCase(authenticateFromToken.fulfilled, (state, action) => {
            state.status = "authenticated";
            state.user = action.payload;
        })
        .addCase(googleSignUp.pending, (state) => {
            state.status = "pending";
        })
        .addCase(googleSignUp.rejected, (state) => {
            state.status = "authentication-failed";
        })
        .addCase(googleSignUp.fulfilled, (state, action) => {
            state.status = "authenticated";
            state.user = action.payload;
        })
        .addCase(facebookSignIn.pending, (state) => {
            state.status = "pending";
        })
        .addCase(facebookSignIn.rejected, (state) => {
            state.status = "authentication-failed";
        })
        .addCase(facebookSignIn.fulfilled, (state, action) => {
            state.status = "authenticated";
            state.user = action.payload;
        })
        .addCase(facebookSignUp.pending, (state) => {
            state.status = "pending";
        })
        .addCase(facebookSignUp.rejected, (state) => {
            state.status = "authentication-failed";
        })
        .addCase(facebookSignUp.fulfilled, (state, action) => {
            state.status = "authenticated";
            state.user = action.payload;
        })
})

export const authReducer = slice.reducer;

const { signOut } = slice.actions;

export const AuthActions = {
    signOut,
    googleSignIn,
    googleSignUp,
    authenticateFromToken,
    facebookSignIn,
    facebookSignUp
}

const createSelector = createSelectorCreator(state => state.auth);

export const AuthSelectors = {
    selectStatus: createSelector(state => state.status),
    selectUser: createSelector(state => state.user),
    selectUserUnsafe: createSelector(state => state.user!)
}