import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import BackButton from "src/components/base/back-button";
import { Text } from "src/components/base/text";
import { useTranslation } from "react-i18next";
import HeadingLayout from "src/components/base/heading-layout";
import { Table, TableBody, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Api } from "src/api/api";
import { Button } from "src/components/base/button";

/**
 * The properties for {@link UserOverview}
 */
export type UserDashboardProps = {};

/**
 * The overview of the club users
 */
function UserOverview(props: UserDashboardProps) {
    const [t] = useTranslation();
    const [tU] = useTranslation("club-admin-user-overview");

    const [clubUsers, setClubUsers] = React.useState([]);

    /**
     * Refresh club users
     */
    const refreshClubUsers = async () => {
        const res = await Api.clubAdmin;
    };

    useEffect(() => {
        refreshClubUsers().then();
    }, []);

    return (
        <>
            <BackButton href={"/ca/dashboard"}>
                <Text>{tU("button.back-to-dashboard")}</Text>
            </BackButton>
            <HeadingLayout
                className={"mt-6"}
                heading={tU("heading.user-overview")}
                headingChildren={<Button href={"/ca/users/create"}>{tU("button.create-user")}</Button>}
            >
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableHeader>{t("label.username")}</TableHeader>
                            <TableHeader>{t("label.display-name")}</TableHeader>
                            <TableHeader>{tU("label.website-count")}</TableHeader>
                            <TableHeader className={"w-0"}>
                                <span className={"sr-only"}>{t("accessibility.actions")}</span>
                            </TableHeader>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        <TableRow></TableRow>
                    </TableBody>
                </Table>
            </HeadingLayout>
        </>
    );
}

export const Route = createFileRoute("/_club-admin/ca/users/")({
    component: UserOverview,
});
