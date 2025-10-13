import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import { Api } from "src/api/api";
import { useTranslation } from "react-i18next";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Badge } from "src/components/base/badge";

/**
 * The properties for {@link AdminClubDomains}
 */
export type AdminClubDomainsProps = {};

/**
 * Domains for a single club
 */
export default function AdminClubDomains(props: AdminClubDomainsProps) {
    const [t] = useTranslation("admin-club-view");

    const { associated } = Route.useLoaderData();

    return (
        <div className={"flex flex-col gap-6"}>
            <Table>
                <TableHead>
                    <TableRow>
                        <TableHeader>{t("label.domain")}</TableHeader>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {associated
                        .sort((a, b) => (a.is_primary && -1) || (b.is_primary && 1) || 0)
                        .map((item) => (
                            <TableRow key={item.uuid}>
                                <TableCell>
                                    <div className={"flex gap-12"}>
                                        <span>{item.domain}</span>
                                        {item.is_primary && <Badge color={"green"}>{t("label.primary-domain")}</Badge>}
                                    </div>
                                </TableCell>
                            </TableRow>
                        ))}
                </TableBody>
            </Table>
        </div>
    );
}

export const Route = createFileRoute("/_menu/a/clubs/$clubId/_club/domains")({
    component: AdminClubDomains,
    loader: async ({ params }) => ({
        associated: await Api.admin.clubs.associatedDomains(params.clubId),
    }),
});
