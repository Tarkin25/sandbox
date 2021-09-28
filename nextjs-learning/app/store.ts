import { configureStore, createReducer, Store } from "@reduxjs/toolkit";
import { createWrapper, Context } from "next-redux-wrapper";

const countReducer = createReducer(
    0,
    builder => builder
)

export type RootState = ReturnType<typeof countReducer>;

export const makeStore = (context: Context) => {
    const store = configureStore({
        reducer: countReducer
    })

    return store;
}

export const wrapper = createWrapper<Store<RootState>>(makeStore);