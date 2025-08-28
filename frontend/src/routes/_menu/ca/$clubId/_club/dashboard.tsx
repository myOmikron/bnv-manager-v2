import { createFileRoute } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
/**
 * The properties for {@link ClubAdminDashboard}
 */
export type ClubAdminDashboardProps = {};

/**
 * Dashboard for admins that show a single club
 */
export default function ClubAdminDashboard(props: ClubAdminDashboardProps) {
    const [t] = useTranslation("ca-club-view");

    return <div></div>;
}

export const Route = createFileRoute("/_menu/ca/$clubId/_club/dashboard")({
    component: ClubAdminDashboard,
});
