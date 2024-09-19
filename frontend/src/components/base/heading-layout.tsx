import React from "react";
import { Heading } from "./heading";
import { clsx } from "clsx";

/**
 * The properties for {@link HeadingLayout}
 */
export type HeadingLayoutProps = {
    /** The text for the heading */
    heading: string;

    /** Additional children that will be displayed in the heading */
    headingChildren?: Array<React.ReactNode> | React.ReactNode;

    /** Everything below the heading */
    children?: React.ReactNode;

    /** Set additional classes */
    className?: string;
};

/**
 * A layout that includes a top level heading
 */
export default function HeadingLayout(props: HeadingLayoutProps) {
    return (
        <div className={clsx("flex flex-col gap-8", props.className)}>
            <div className="flex w-full flex-wrap items-end justify-between gap-4 border-b border-zinc-950/10 pb-6 dark:border-white/10">
                <Heading>{props.heading}</Heading>
                {props.headingChildren !== undefined ? (
                    <div className={"flex gap-4"}>{props.headingChildren}</div>
                ) : undefined}
            </div>
            {props.children}
        </div>
    );
}
