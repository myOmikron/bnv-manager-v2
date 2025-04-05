import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import { useTranslation } from "react-i18next";

/**
 * The properties for {@link AdminClubDomains}
 */
export type AdminClubDomainsProps = {};

/**
 * Domains for a single club
 */
export default function AdminClubDomains(props: AdminClubDomainsProps) {
    const [t] = useTranslation("admin-club-view");

    return <div></div>;
}

export const Route = createFileRoute("/_menu/a/clubs/$clubId/_club/domains")({
    component: AdminClubDomains,
});
