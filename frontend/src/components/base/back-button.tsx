import React from "react";
import { Button } from "src/components/base/button";
import ChevronLeftIcon from "@heroicons/react/20/solid/ChevronLeftIcon";

/**
 * The properties for {@link BackButton}
 */
export type BackButtonProps = {
    /** Children */
    children: React.ReactNode;
    /** onClick action */
    onClick?: () => void;
};

/**
 * Button to go back
 */
export default function BackButton(props: BackButtonProps) {
    return (
        <Button plain={true} onClick={props.onClick}>
            <ChevronLeftIcon />
            {props.children}
        </Button>
    );
}
