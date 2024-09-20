import { createFileRoute } from "@tanstack/react-router";

import React from "react";

/**
 * The properties for {@link CreateWebsite}
 */
export type CreateWebsiteProps = {};

/**
 * Create a website
 */
export default function CreateWebsite(props: CreateWebsiteProps) {
    return <div></div>;
}

export const Route = createFileRoute("/_user/u/websites/create")({
    component: CreateWebsite,
});
