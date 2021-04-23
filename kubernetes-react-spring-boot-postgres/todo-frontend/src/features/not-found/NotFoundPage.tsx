import {
    AppBar,
    makeStyles,
    Theme,
    Toolbar,
    Typography,
    useTheme,
} from "@material-ui/core";
import { grey } from "@material-ui/core/colors";
import React, { useEffect } from "react";
import { useParams } from "react-router";
import { ReactTerminal, TerminalContextProvider } from "react-terminal";
import { openWindows, CommandMapBuilder, download } from "./Command";

interface Params {
    path: string;
}

interface TerminalTheme {
    themeBGColor: string;
    themeToolbarColor: string;
    themeColor: string;
    themePromptColor: string;
}

interface ThemeMap {
    custom: TerminalTheme;
}

const getTitleFromPath = (path: string) => {
    const first = path.split("/")[0];

    return first.substring(0, 1).toLocaleUpperCase() + first.substring(1);
};

const commands = new CommandMapBuilder()
    .add("echo", "Prints any input", (args) => args)
    .add("test", "Just try it!", () => {
        window.open("https://pornhub.com")
    })
    .add("mysql", "Open a MySQL connection", download("https://freepikpsd.com/wp-content/uploads/2019/11/transparent-png-example-Images.png", "example.png"))
    .add("window", "Annoying window", openWindows(10, "https://google.com/search?q=You+have+been+hacked"))
.build()

const welcomeMessage = (title: string) => (
    <>
        {`Welcome to ${title} v6.9.42.0.`}
        <br />
        <br />
        Type "help" to display a list of commands.
        <br />
        <br />
    </>
);

const themeMap = (theme: Theme): ThemeMap => ({
    custom: {
        themeBGColor: theme.palette.background.default,
        themeColor: grey[400],
        themePromptColor: theme.palette.primary.main,
        themeToolbarColor: "transparent",
    },
});

const useStyles = makeStyles(
    (theme) => ({
        root: {
            display: "flex",
            flexDirection: "column",
            height: "100vh",
        },
        terminalWrapper: {
            flexGrow: 1,
            "& .index_terminal__1dSq1": {
                background: "inherit !important",
                "& .index_controls__9z-dM": {
                    display: "none",
                },
                "& .index_editor__2tqRz": {
                    height: "100%",
                },
            },
        },
    }),
    { name: "NotFoundPage" }
);

const NotFoundPage = () => {
    const { path } = useParams<Params>();
    const title = getTitleFromPath(path);
    const classes = useStyles();
    const theme = useTheme();
    const openSelf = window.location.href.includes("?openSelf=true");

    useEffect(() => {
        if(openSelf) {
            window.open(window.location.href)
        }
    }, [openSelf]);

    return (
        <div className={classes.root}>
            <AppBar position="static">
                <Toolbar>
                    <Typography variant="h4">{title}</Typography>
                </Toolbar>
            </AppBar>
            <div className={classes.terminalWrapper}>
                <TerminalContextProvider>
                    <ReactTerminal
                        welcomeMessage={welcomeMessage(title)}
                        themes={themeMap(theme)}
                        theme="custom"
                        showControlButtons={false}
                        commands={commands}
                        prompt="$"
                    />
                </TerminalContextProvider>
            </div>
        </div>
    );
};

export default NotFoundPage;
