import { makeStyles } from '@material-ui/core';
import React, { Fragment, ReactNode } from 'react'

export type CommandHelp = {
    name: string;
    description: ReactNode;
}

export type HelpDisplayProps = {
    commands: CommandHelp[]
}

const useStyles = makeStyles(theme => ({
    root: {
        paddingTop: theme.spacing(2),
        display: "grid",
        gridTemplateColumns: `auto auto`,
        columnGap: theme.spacing(2)
    }
}), { name: "HelpDisplay" });

const HelpDisplay = (props: HelpDisplayProps) => {

    const {commands} = props;
    const classes = useStyles();

    return (
        <div className={classes.root}>
            {commands.map(({name, description}) => (
                <Fragment key={name}>
                    <div>
                        {name}
                    </div>
                    <div>
                        {description}
                    </div>
                </Fragment>
            ))}
        </div>
    )
}

export default HelpDisplay
