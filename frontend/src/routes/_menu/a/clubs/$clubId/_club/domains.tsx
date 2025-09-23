import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import { Api } from "src/api/api";

/**
 * The properties for {@link AdminClubDomains}
 */
export type AdminClubDomainsProps = {};

/**
 * Domains for a single club
 */
export default function AdminClubDomains(props: AdminClubDomainsProps) {
    //const [t] = useTranslation("admin-club-view");

    const { associated } = Route.useLoaderData();

    return <div className={"flex flex-col gap-6"}>{JSON.stringify([associated])}</div>;
}

export const Route = createFileRoute("/_menu/a/clubs/$clubId/_club/domains")({
    component: AdminClubDomains,
    loader: async ({ params }) => ({
        associated: await Api.admin.clubs.associatedDomains(params.clubId),
    }),
});
