import React from "react";
import { CreateTodo, TodoActions } from "./todoSlice";
import { Field, Form, Formik, FormikHelpers } from "formik";
import {
    Grid,
    IconButton,
    ListItem,
    ListItemIcon,
    ListItemSecondaryAction,
} from "@material-ui/core";
import { TextField } from "formik-material-ui";
import { useAppDispatch } from "../../app/hooks";
import AddIcon from "@material-ui/icons/AddCircle";

const initialValues: CreateTodo = {
    title: "",
    description: "",
};

const CreateTodoForm = () => {
    const dispatch = useAppDispatch();

    const handleSubmit = async (
        todo: CreateTodo,
        helpers: FormikHelpers<CreateTodo>
    ) => {
        await dispatch(TodoActions.create(todo));

        helpers.setSubmitting(false);
        helpers.resetForm();
    };

    return (
        <ListItem>
            <ListItemIcon />
            <Formik initialValues={initialValues} onSubmit={handleSubmit}>
                <Form>
                    <Grid container direction="row" spacing={1}>
                        <Grid item>
                            <Field
                                component={TextField}
                                name="title"
                                label="Title"
                                size="small"
                            />
                        </Grid>
                        <Grid item>
                            <Field
                                component={TextField}
                                name="description"
                                label="Description"
                                size="small"
                            />
                        </Grid>
                    </Grid>
                    <ListItemSecondaryAction>
                        <IconButton type="submit" color="primary" size="small">
                            <AddIcon />
                        </IconButton>
                    </ListItemSecondaryAction>
                </Form>
            </Formik>
        </ListItem>
    );
};

export default CreateTodoForm;
