import { createFileRoute, Outlet } from "@tanstack/react-router";

import React from "react";
import { useTranslation } from "react-i18next";
import CLUB_ADMIN_SINGLE_CLUB, { ClubAdminSingleClubProvider } from "src/context/club-admin-single-club";
import TabLayout from "src/components/base/tab-layout";
import { Tab, TabMenu } from "src/components/base/tab-menu";

/**
 * The properties for {@link ClubAdminClub}
 */
export type ClubAdminClubProps = {};

/**
 * Wrapper to provider the context for the club admin view
 */
export default function ClubAdminClub(props: ClubAdminClubProps) {
    const [t] = useTranslation("ca-club-view");

    const params = Route.useParams();

    return (
        <>
            <ClubAdminSingleClubProvider uuid={params.clubId}>
                <CLUB_ADMIN_SINGLE_CLUB.Consumer>
                    {(ctx) => (
                        <TabLayout
                            heading={t("heading.club", { club: ctx.data.name })}
                            tabs={
                                <TabMenu>
                                    <Tab href={"/ca/$clubId/dashboard"} params={{ clubId: params.clubId }}>
                                        {t("heading.club-dashboard")}
                                    </Tab>
                                </TabMenu>
                            }
                        >
                            <Outlet />
                        </TabLayout>
                    )}
                </CLUB_ADMIN_SINGLE_CLUB.Consumer>
            </ClubAdminSingleClubProvider>
        </>
    );
}

export const Route = createFileRoute("/_menu/ca/$clubId/_club")({
    component: ClubAdminClub,
});
