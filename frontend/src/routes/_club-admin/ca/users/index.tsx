import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import BackButton from "src/components/base/back-button";
import { Text } from "src/components/base/text";
import { useTranslation } from "react-i18next";
import HeadingLayout from "src/components/base/heading-layout";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Api, UUID } from "src/api/api";
import { Button, ExternalButton } from "src/components/base/button";
import { toast } from "react-toastify";
import { SimpleUser } from "src/api/generated";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { ArrowDownOnSquareStackIcon, EllipsisVerticalIcon } from "@heroicons/react/24/outline";
import { Field, Label } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";
import ConfirmDialog from "src/components/confirm-dialog";

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

    const [clubUsers, setClubUsers] = React.useState<Array<SimpleUser>>([]);
    const [searchQuery, setSearchQuery] = React.useState("");

    const [openDeleteUser, setOpenDeleteUser] = React.useState<SimpleUser>();

    const deleteUser = async (uuid: UUID) => {
        const res = await Api.clubAdmin.users.delete(uuid);

        res.match(
            (ok) => {},
            (err) => toast.error(err.message),
        );
    };

    /**
     * Refresh club users
     */
    const refreshClubUsers = async () => {
        const res = await Api.clubAdmin.users.all();

        res.match(
            (users) => setClubUsers(users),
            (err) => toast.error(err.message),
        );
    };

    useEffect(() => {
        refreshClubUsers().then();
    }, []);

    const filtered =
        searchQuery === ""
            ? clubUsers
            : clubUsers.filter(
                  (u) =>
                      u.username.toLowerCase().includes(searchQuery.toLowerCase()) ||
                      u.display_name.toLowerCase().includes(searchQuery.toLowerCase()),
              );

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
                <div className={"flex items-end justify-between gap-12"}>
                    <Field>
                        <Label>{t("label.search")}</Label>
                        <Input value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} />
                    </Field>

                    <div className={"flex gap-3"}>
                        <ExternalButton outline={true} href={"/api/frontend/v1/club-admin/users/export/csv"}>
                            <span>{tU("button.export-csv")}</span>
                            <ArrowDownOnSquareStackIcon className={"!size-5"} />
                        </ExternalButton>
                        <ExternalButton outline={true} href={"/api/frontend/v1/club-admin/users/export/json"}>
                            <span>{tU("button.export-json")}</span>
                            <ArrowDownOnSquareStackIcon className={"!size-5"} />
                        </ExternalButton>
                    </div>
                </div>

                <Table dense={true}>
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
                        {filtered.map((x) => (
                            <TableRow key={x.uuid}>
                                <TableCell>{x.username}</TableCell>
                                <TableCell>{x.display_name}</TableCell>
                                <TableCell>{x.website_count}</TableCell>
                                <TableCell>
                                    <Dropdown>
                                        <DropdownButton plain={true}>
                                            <EllipsisVerticalIcon />
                                        </DropdownButton>

                                        <DropdownMenu anchor={"bottom end"}>
                                            <DropdownItem>
                                                <DropdownLabel>{tU("button.create-reset-link")}</DropdownLabel>
                                            </DropdownItem>
                                            <DropdownItem onClick={() => setOpenDeleteUser(x)}>
                                                <DropdownLabel>{t("button.delete")}</DropdownLabel>
                                            </DropdownItem>
                                        </DropdownMenu>
                                    </Dropdown>
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </HeadingLayout>

            {openDeleteUser && (
                <ConfirmDialog
                    title={tU("heading.confirm-delete-user", { user: openDeleteUser.username })}
                    description={tU("description.confirm-delete-user")}
                    onConfirm={async () => {
                        if (openDeleteUser) {
                            await deleteUser(openDeleteUser.uuid);
                            await refreshClubUsers();
                            setOpenDeleteUser(undefined);
                        }
                    }}
                    onCancel={() => setOpenDeleteUser(undefined)}
                />
            )}
        </>
    );
}

export const Route = createFileRoute("/_club-admin/ca/users/")({
    component: UserOverview,
});
