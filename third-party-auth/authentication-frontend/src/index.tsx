import React from "react";
import ReactDOM from "react-dom";
import App from "./app/App";
import { store } from "./app/store";
import { Provider } from "react-redux";
import { CssBaseline, MuiThemeProvider } from "@material-ui/core";
import theme from "./resources/theme";

ReactDOM.render(
    <React.StrictMode>
        <Provider store={store}>
            <MuiThemeProvider theme={theme}>
                <CssBaseline />
                <App />
            </MuiThemeProvider>
        </Provider>
    </React.StrictMode>,
    document.getElementById("root")
);
