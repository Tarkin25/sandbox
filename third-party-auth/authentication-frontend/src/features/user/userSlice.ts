import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import api from "../../app/api";

export interface User {
    id: number;
    name: string;
    email: string;
}

const initialState: User[] = [];

const fetchAll = createAsyncThunk(
    "users/fetchAll",
    async () => {
        const res = await api.get<User[]>("/users");

        return res.data;
    }
)

const slice = createSlice({
    name: "users",
    initialState,
    reducers: {},
    extraReducers: builder => builder
        .addCase(fetchAll.fulfilled, (state, action) => {
            return action.payload;
        })
})

export const userReducer = slice.reducer;

export const UserActions = {
    fetchAll
}