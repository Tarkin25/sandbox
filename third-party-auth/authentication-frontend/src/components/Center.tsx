import { makeStyles } from '@material-ui/core'
import React, { ReactNode } from 'react'

export type CenterProps = {
    children?: ReactNode;
}

const useStyles = makeStyles(theme => ({
    root: {
        position: "static",
        width: "100%",
        height: "100%"
    },
    content: {
        position: "absolute",
        top: "50%",
        left: "50%",
        transform: "translate(-50%, -50%)"
    }
}), { name: "Center" });

const Center = (props: CenterProps) => {

    const classes = useStyles();

    return (
        <div className={classes.root}>
            <div className={classes.content}>
                {props.children}
            </div>
        </div>
    )
}

export default Center
