import React from "react";
import { Heading } from "./heading";
import { Text } from "./text";

/**
 * The properties for {@link TabLayout}
 */
export type TabLayoutProps = {
    /** The heading to display */
    heading: string;
    /** The description of the heading */
    headingDescription?: string;
    /** The tabs to render */
    tabs: React.ReactNode;
    /** The content of the site */
    children: React.ReactNode;
};

/**
 * A layout for tabs
 */
export default function TabLayout(props: TabLayoutProps) {
    return (
        <>
            <Heading className={"mb-6"}>{props.heading}</Heading>
            {props.headingDescription && <Text className={"mb-6"}>{props.headingDescription}</Text>}

            {props.tabs}
            <div className={"mt-6"}>{props.children}</div>
        </>
    );
}
