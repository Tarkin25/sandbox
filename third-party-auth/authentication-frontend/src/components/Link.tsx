import React, { MouseEvent } from 'react'
import { useHistory } from "react-router-dom";
import { Link as MuiLink, LinkProps as MuiLinkProps } from "@material-ui/core";

export type LinkProps = Omit<MuiLinkProps, "href"> & {
    to?: string;
    active?: boolean;
    onClick?: () => void;
}

const Link = (props: LinkProps) => {

    const { to, onClick, className, active, underline, ...rest } = props;
    const history = useHistory();

    const handleClick = (e: MouseEvent) => {
        e.preventDefault();

        if(onClick) {
            onClick();
        } else if (to) {
            history.push(to);
        }
    }

    return (
        <MuiLink href={!onClick ? to : undefined} onClick={handleClick} underline={active ? "always" : underline} {...rest} />
    )
}

export default Link
