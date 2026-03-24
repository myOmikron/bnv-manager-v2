import { createFileRoute } from "@tanstack/react-router";
import { useContext } from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import { Text, Strong } from "src/components/base/text";
import { UsersIcon, ShieldCheckIcon, EnvelopeIcon, GlobeAltIcon, CalendarIcon } from "@heroicons/react/20/solid";
import CLUB_ADMIN_SINGLE_CLUB from "src/context/club-admin-single-club";

/**
 * The properties for {@link ClubAdminDashboard}
 */
export type ClubAdminDashboardProps = {};

/**
 * Dashboard for admins that show a single club
 */
export default function ClubAdminDashboard(props: ClubAdminDashboardProps) {
    const [t] = useTranslation("ca-club-view");

    const ctx = useContext(CLUB_ADMIN_SINGLE_CLUB);
    const pendingInvites = Route.useLoaderData();

    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"grid grid-cols-1 gap-4 sm:grid-cols-3"}>
                <StatCard
                    icon={<UsersIcon className={"size-5 text-blue-500"} />}
                    label={t("dashboard.members")}
                    value={ctx.data.member_count}
                />
                <StatCard
                    icon={<ShieldCheckIcon className={"size-5 text-emerald-500"} />}
                    label={t("dashboard.admins")}
                    value={ctx.data.admin_count}
                />
                <StatCard
                    icon={<EnvelopeIcon className={"size-5 text-amber-500"} />}
                    label={t("dashboard.pending-invites")}
                    value={pendingInvites.length}
                />
            </div>

            <div className={"flex flex-col gap-3"}>
                <div className={"flex items-center gap-2"}>
                    <GlobeAltIcon className={"size-4 text-zinc-400"} />
                    <Text>
                        <Strong>{t("dashboard.domain")}</Strong> {ctx.data.primary_domain}
                    </Text>
                </div>
                <div className={"flex items-center gap-2"}>
                    <CalendarIcon className={"size-4 text-zinc-400"} />
                    <Text>
                        <Strong>{t("dashboard.created-at")}</Strong>{" "}
                        {new Date(ctx.data.created_at).toLocaleDateString("de", {
                            day: "2-digit",
                            month: "2-digit",
                            year: "numeric",
                        })}
                    </Text>
                </div>
            </div>
        </div>
    );
}

function StatCard(props: { icon: React.ReactNode; label: string; value: number }) {
    return (
        <div className={"rounded-lg border border-zinc-200 p-4 dark:border-zinc-700"}>
            <div className={"flex items-center gap-2"}>
                {props.icon}
                <Text>{props.label}</Text>
            </div>
            <div className={"mt-2 text-2xl font-semibold text-zinc-950 dark:text-white"}>{props.value}</div>
        </div>
    );
}

export const Route = createFileRoute("/_menu/ca/$clubId/_club/dashboard")({
    component: ClubAdminDashboard,
    loader: async ({ params }) => await Api.clubAdmins.club.getInvitedMembers(params.clubId),
});
