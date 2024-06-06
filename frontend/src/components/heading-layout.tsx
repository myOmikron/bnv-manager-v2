import React from "react";
import { Heading } from "./heading";
import { clsx } from "clsx";

/**
 * The properties for {@link HeadingLayout}
 */
export type HeadingLayoutProps = {
    /** The text for the heading */
    heading: string;
    /** Disable the bottom border of the heading */
    disableBottomBorder?: boolean;
    /** The buttons that should be shown in the heading */
    headingButtons?: React.ReactNode;
    /** The notes that should be rendered below the heading */
    children?: React.ReactNode;
};

/**
 * A layout with a top heading
 */
export default function HeadingLayout(props: HeadingLayoutProps) {
    return (
        <div className={"flex w-full flex-col gap-6"}>
            <div
                className={clsx(
                    "flex w-full flex-wrap items-end justify-between gap-4 border-zinc-950/10 pb-6 dark:border-white/10",
                    props.disableBottomBorder ?? "border-b",
                )}
            >
                <Heading>{props.heading}</Heading>
                {props.headingButtons}
            </div>
            {props.children}
        </div>
    );
}
