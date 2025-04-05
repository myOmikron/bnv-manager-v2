import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import HeadingLayout from "src/components/base/heading-layout";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Dropdown, DropdownButton, DropdownMenu } from "src/components/base/dropdown";
import { EllipsisVerticalIcon } from "@heroicons/react/20/solid";

/**
 * The properties for {@link AdminOverview}
 */
export type AdminOverviewProps = {};

/**
 * An overview of all admin accounts
 */
export default function AdminOverview(props: AdminOverviewProps) {
    const [t] = useTranslation("admin-overview");
    const [tg] = useTranslation();

    const data = Route.useLoaderData();

    return <HeadingLayout heading={t("heading.admin-overview")}>

        <Table dense={true}>
            <TableHead>
                <TableHeader>{t("label.username")}</TableHeader>
                <TableHeader>{t("label.display-name")}</TableHeader>
                <TableHeader className={"w-0"}>
                    <span className={"sr-only"}>{tg("accessibility.actions")}</span>
                </TableHeader>
            </TableHead>
            <TableBody>
                {data.map((user) => <TableRow key={user.uuid}>
                    <TableCell>{user.username}</TableCell>
                    <TableCell>{user.display_name}</TableCell>
                    <TableCell>
                        <Dropdown>
                            <DropdownButton plain={true}>
                                <EllipsisVerticalIcon />
                            </DropdownButton>
                            <DropdownMenu anchor={"bottom end"}></DropdownMenu>
                        </Dropdown>
                    </TableCell>
                </TableRow>)}
            </TableBody>
        </Table>

    </HeadingLayout>;
}


export const Route = createFileRoute("/_menu/a/admins/")({
    component: AdminOverview,
    // eslint-disable-next-line
    loader: async () => await Api.admin.admins.getAll()
});
