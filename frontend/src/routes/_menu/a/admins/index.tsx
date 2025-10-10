import { createFileRoute } from "@tanstack/react-router";

import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import HeadingLayout from "src/components/base/heading-layout";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { AtSymbolIcon, ChevronDownIcon, EllipsisVerticalIcon, UserPlusIcon } from "@heroicons/react/20/solid";
import DialogCreateAdmin from "src/components/dialogs/admin-create-admin";

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

    const [openCreateAdmin, setOpenCreateAdmin] = React.useState(false);
    const [_openAssignAdmin, setOpenAssignAdmin] = React.useState(false);

    return (
        <HeadingLayout
            heading={t("heading.admin-overview")}
            headingChildren={
                <Dropdown>
                    <DropdownButton outline={true}>
                        <span>{tg("button.options")}</span>
                        <ChevronDownIcon />
                    </DropdownButton>
                    <DropdownMenu anchor={"bottom end"}>
                        <DropdownItem onClick={() => setOpenCreateAdmin(true)}>
                            <UserPlusIcon />
                            <DropdownLabel>{t("button.create-admin")}</DropdownLabel>
                        </DropdownItem>
                        <DropdownItem onClick={() => setOpenAssignAdmin(true)}>
                            <AtSymbolIcon />
                            <DropdownLabel>{t("button.assign-admin")}</DropdownLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            }
        >
            <Table dense={true}>
                <TableHead>
                    <TableRow>
                        <TableHeader>{t("label.username")}</TableHeader>
                        <TableHeader>{t("label.display-name")}</TableHeader>
                        <TableHeader className={"w-0"}>
                            <span className={"sr-only"}>{tg("accessibility.actions")}</span>
                        </TableHeader>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {data.map((user) => (
                        <TableRow key={user.uuid}>
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
                        </TableRow>
                    ))}
                </TableBody>
            </Table>

            <Suspense>
                <DialogCreateAdmin open={openCreateAdmin} onClose={() => setOpenCreateAdmin(false)} />
            </Suspense>
        </HeadingLayout>
    );
}

export const Route = createFileRoute("/_menu/a/admins/")({
    component: AdminOverview,
    loader: async () => await Api.admin.superadmins.getAll(),
});
