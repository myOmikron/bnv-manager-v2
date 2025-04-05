import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import { useTranslation } from "react-i18next";

/**
 * The properties for {@link AdminClubDashboard}
 */
export type AdminClubDashboardProps = {};

/**
 * Dashboard for admins that show a single club
 */
export default function AdminClubDashboard(props: AdminClubDashboardProps) {
    const [t] = useTranslation("admin-club-view");

    return <div></div>;
}

export const Route = createFileRoute("/_menu/a/clubs/$clubId/_club/dashboard")({
    component: AdminClubDashboard,
});
