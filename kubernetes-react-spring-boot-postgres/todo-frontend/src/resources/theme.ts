import { createMuiTheme } from "@material-ui/core";
import { purple } from "@material-ui/core/colors";

const theme = createMuiTheme({
    palette: {
        type: "dark",
        primary: {
            main: purple[400]
        }
    },
    overrides: {
        MuiAppBar: {
            colorPrimary: {
                backgroundColor: purple[600]
            }
        }
    }
})

export default theme;