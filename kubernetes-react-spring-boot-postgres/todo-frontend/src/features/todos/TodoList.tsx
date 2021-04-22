import {
    Checkbox,
    Divider,
    IconButton,
    List,
    ListItem,
    ListItemIcon,
    ListItemSecondaryAction,
    ListItemText,
} from "@material-ui/core";
import React, { Fragment, useEffect } from "react";
import { useAppDispatch, useAppSelector } from "../../app/hooks";
import CreateTodoForm from "./CreateTodoForm";
import Todo from "./Todo";
import { TodoActions, TodoSelectors } from "./todoSlice";
import DeleteIcon from "@material-ui/icons/DeleteOutlined";

const TodoList = () => {
    const todos = useAppSelector(TodoSelectors.selectAll);
    const dispatch = useAppDispatch();

    useEffect(() => {
        dispatch(TodoActions.fetchAll());
    }, [dispatch]);

    const mapTodo = (todo: Todo) => {
        const { id, title, description, completed } = todo;
        const complete = () => dispatch(TodoActions.toggleCompleted(todo));
        const handleDelete = () => dispatch(TodoActions.deleteById(id));

        return (
            <Fragment key={id}>
                <ListItem>
                <ListItemIcon>
                <Checkbox color="primary" checked={completed} onClick={complete} />
                </ListItemIcon>
                <ListItemText primary={title} secondary={description} />
                <ListItemSecondaryAction>
                    <IconButton onClick={handleDelete} size="small">
                        <DeleteIcon color="primary" />
                    </IconButton>
                </ListItemSecondaryAction>
            </ListItem>
            <Divider variant="inset" />
            </Fragment>
        );
    };

    return (
        <div>
            <List>
                {todos.map(mapTodo)}
                <CreateTodoForm />
            </List>
        </div>
    );
};

export default TodoList;
