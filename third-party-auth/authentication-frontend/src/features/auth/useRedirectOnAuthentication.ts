import { useEffect } from "react";
import { useSelector } from "react-redux";
import { useHistory } from "react-router";
import Routes from "../../app/Routes";
import { AuthSelectors } from "./authSlice";

const useRedirectOnAuthentication = () => {
    const status = useSelector(AuthSelectors.selectStatus);
    const history = useHistory();

    useEffect(() => {
        if(status === "authenticated") {
            history.push(Routes.HOME);
        }
    }, [status, history]);
}

export default useRedirectOnAuthentication;