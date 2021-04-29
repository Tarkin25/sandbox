import React from "react";
import { useSelector } from "react-redux";
import {
    Redirect,
    Route as DomRoute,
    RouteProps as DomRouteProps,
} from "react-router-dom";
import { AuthSelectors } from "../features/auth/authSlice";
import Routes from "./Routes";

export type RouteProps = DomRouteProps & {
    secure?: boolean;
};

const Route = (props: RouteProps) => {
    const { secure, ...rest } = props;
    const status = useSelector(AuthSelectors.selectStatus);

    if (secure) {
        if (status === "authenticated") {
            return <DomRoute {...rest} />;
        } else {
            return <Redirect to={Routes.SIGN_IN} />;
        }
    } else {
        return <DomRoute {...rest} />;
    }
};

export default Route;
