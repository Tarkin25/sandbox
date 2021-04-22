import { createAsyncThunk, createEntityAdapter, createSlice } from "@reduxjs/toolkit";
import api from "../../app/api";
import { AppThunk, RootState } from "../../app/store";
import Todo from "./Todo";

const adapter = createEntityAdapter<Todo>({
    sortComparer: (a, b) => {
        if(a.completed === b.completed) {
            return a.id - b.id;
        } else {
            return a.completed ? 1 : -1;
        }
    }
});

const initialState = adapter.getInitialState();

export type TodoState = typeof initialState;

const fetchAll = createAsyncThunk(
    "todos/fetchAll",
    async () => {
        const res = await api.get<Todo[]>("/todos");

        return res.data;
    }
)

export type CreateTodo = Pick<Todo, "title" | "description">

const create = createAsyncThunk(
    "todos/create",
    async (todo: CreateTodo) => {
        const res = await api.post<Todo>("/todos", todo);

        return res.data;
    }
)

const update = createAsyncThunk(
    "todos/update",
    async (todo: Todo) => {
        const res = await api.put<Todo>(`/todos/${todo.id}`, todo);

        return res.data;
    }
)

const toggleCompleted = (todo: Todo): AppThunk => (dispatch) => {
    dispatch(update({...todo, completed: !todo.completed}));
}

const deleteById = createAsyncThunk(
    "todos/deleteById",
    async (id: number) => {
        const res = await api.delete<void>(`/todos/${id}`);

        return res.data;
    }
)

export const TodoActions = {
    fetchAll,
    create,
    toggleCompleted,
    deleteById
}

const slice = createSlice({
    name: "todos",
    initialState,
    reducers: {},
    extraReducers: builder => builder
        .addCase(fetchAll.fulfilled, (state, {payload}) => {
            adapter.setAll(state, payload);
        })
        .addCase(create.fulfilled, (state, {payload}) => {
            adapter.addOne(state, payload);
        })
        .addCase(update.fulfilled, (state, {payload}) => {
            adapter.upsertOne(state, payload);
        })
        .addCase(deleteById.fulfilled, (state, {meta: {arg}}) => {
            adapter.removeOne(state, arg);
        })
})

export const todoReducer = slice.reducer;

/**
 * SELECTORS
 */

export const TodoSelectors = adapter.getSelectors((state: RootState) => state.todos);