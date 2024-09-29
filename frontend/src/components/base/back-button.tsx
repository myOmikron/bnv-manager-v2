import React from "react";
import { Button } from "src/components/base/button";
import ChevronLeftIcon from "@heroicons/react/20/solid/ChevronLeftIcon";
import { LinkProps } from "src/components/base/link";

/**
 * The properties for {@link BackButton}
 */
export type BackButtonProps = {
    /** Children */
    children: React.ReactNode;
    /** onClick action */
    onClick?: () => void;
    /** A href to transform the button into a link */
    href?: LinkProps["href"];
    /** params for the link */
    params?: LinkProps["params"];
    /** The search for the link */
    search?: LinkProps["search"];
};

/**
 * Button to go back
 */
export default function BackButton(props: BackButtonProps) {
    return (
        <Button href={props.href} params={props.params} search={props.search} plain={true} onClick={props.onClick}>
            <ChevronLeftIcon />
            {props.children}
        </Button>
    );
}
