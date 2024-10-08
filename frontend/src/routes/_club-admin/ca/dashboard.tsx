import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import Stats from "src/components/base/stats";
import { useTranslation } from "react-i18next";
import USER_CONTEXT from "src/context/user";
import { Heading } from "src/components/base/heading";
import { Button } from "src/components/base/button";
import { Api } from "src/api/api";
import { FullClub } from "src/api/generated";
import { toast } from "react-toastify";
import { Text } from "src/components/base/text";

/**
 * The properties for {@link ClubAdminDashboard}
 */
export type ClubAdminDashboardProps = {};

/**
 * Club admin dashboard
 */
function ClubAdminDashboard(props: ClubAdminDashboardProps) {
    const [t] = useTranslation();
    const [tD] = useTranslation("club-admin-dashboard");

    const { user } = React.useContext(USER_CONTEXT);

    const [club, setClub] = React.useState<FullClub>();

    /**
     * Refresh the club
     */
    const refreshClub = async () => {
        const res = await Api.clubAdmin.clubs.get();

        res.match(
            (club) => setClub(club),
            (err) => toast.error(err.message),
        );
    };

    useEffect(() => {
        refreshClub().then();
    }, []);

    if (!club) {
        return <Text>{t("label.loading")}</Text>;
    }

    return (
        <div className={"flex flex-col gap-12"}>
            <Heading>{tD("heading.welcome", { user: user.display_name })}</Heading>

            <div className={"grid grid-cols-1 gap-6 lg:grid-cols-3"}>
                <Stats label={tD("label.user-count")} value={club.user_count} />
                <Stats label={tD("label.website-count")} value={club.website_count} />
                <Stats label={tD("label.mail-domain")} value={club.domain} />
            </div>

            <div className={"mt-6 grid grid-cols-1 gap-6 lg:grid-cols-3"}>
                <Button color={"dark/white"} href={"/ca/users/create"}>
                    {tD("button.create-user")}
                </Button>
            </div>
        </div>
    );
}

export const Route = createFileRoute("/_club-admin/ca/dashboard")({
    component: ClubAdminDashboard,
});
