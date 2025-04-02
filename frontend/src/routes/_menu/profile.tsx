import { createFileRoute } from "@tanstack/react-router";

import React from "react";

/**
 * The properties for {@link Profile}
 */
export type ProfileProps = {};

/**
 * Profile component
 */
function Profile(props: ProfileProps) {
    return <div></div>;
}

export const Route = createFileRoute("/_menu/profile")({
    component: Profile,
});
