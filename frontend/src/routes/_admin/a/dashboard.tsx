import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Api, UUID } from "src/api/api";
import { toast } from "react-toastify";
import { SimpleClub } from "src/api/generated";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { EllipsisVerticalIcon, PlusIcon } from "@heroicons/react/20/solid";
import { Button } from "src/components/base/button";
import ConfirmDialog from "src/components/confirm-dialog";

/**
 * The properties for {@link AdminDashboard}
 */
export type AdminDashboardProps = {};

/**
 * A Dashboard for the admins
 */
function AdminDashboard(props: AdminDashboardProps) {
    const [t] = useTranslation();
    const [tA] = useTranslation("admin-dashboard");

    const [clubs, setClubs] = React.useState<Array<SimpleClub>>([]);

    const [openDeleteClub, setOpenDeleteClub] = React.useState<SimpleClub>();

    /**
     * Refresh the clubs
     */
    const refreshClubs = async () => {
        const res = await Api.admin.clubs.all();

        res.match(
            (clubs) => setClubs(clubs.clubs),
            (err) => toast.error(err.message),
        );
    };

    /**
     * Delete a club
     *
     * @param club the uuid of the club to delete
     */
    const deleteClub = async (club: UUID) => {
        const res = await Api.admin.clubs.delete(club);

        if (res.isOk) {
            toast.success(tA("toast.club-deleted"));
            await refreshClubs();
        } else {
            toast.error(res.err.message);
        }
    };

    useEffect(() => {
        refreshClubs().then();
    }, []);

    return (
        <div className={"grid grid-cols-1 gap-6 lg:grid-cols-3"}>
            <div className={"col-span-3 flex max-h-[600px] flex-col"}>
                <div className={"flex justify-end"}>
                    <Button plain={true} href={"/a/clubs/create"}>
                        <PlusIcon className={"fill-black dark:fill-white"} />
                        <span>{tA("button.create-club")}</span>
                    </Button>
                </div>
                <div className={"h-full overflow-y-auto"}>
                    <Table dense={true} className={"dark:text-zinc-200"}>
                        <TableHead>
                            <TableRow>
                                <TableHeader>{tA("label.club-name")}</TableHeader>
                                <TableHeader>{tA("label.club-users")}</TableHeader>
                                <TableHeader className={"w-0"}>
                                    <span className={"sr-only"}>{t("accessibility.actions")}</span>
                                </TableHeader>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {clubs.map((x) => (
                                <TableRow key={x.uuid} href={"/a/clubs/$clubId/general"} params={{ clubId: x.uuid }}>
                                    <TableCell>{x.name}</TableCell>
                                    <TableCell>{x.user_count}</TableCell>
                                    <TableCell>
                                        <Dropdown>
                                            <DropdownButton plain={true}>
                                                <EllipsisVerticalIcon />
                                            </DropdownButton>
                                            <DropdownMenu anchor={"bottom end"}>
                                                <DropdownItem
                                                    href={"/a/clubs/$clubId/club-admins/create"}
                                                    params={{ clubId: x.uuid }}
                                                >
                                                    <DropdownLabel>{tA("button.add-club-admin")}</DropdownLabel>
                                                </DropdownItem>
                                                <DropdownItem onClick={() => setOpenDeleteClub(x)}>
                                                    <DropdownLabel>{t("button.delete")}</DropdownLabel>
                                                </DropdownItem>
                                            </DropdownMenu>
                                        </Dropdown>
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </div>
            </div>

            {openDeleteClub && (
                <ConfirmDialog
                    title={tA("heading.delete-club", { name: openDeleteClub.name })}
                    description={tA("description.delete-club")}
                    onConfirm={() =>
                        openDeleteClub && deleteClub(openDeleteClub.uuid).then(() => setOpenDeleteClub(undefined))
                    }
                    onCancel={() => setOpenDeleteClub(undefined)}
                />
            )}
        </div>
    );
}

export const Route = createFileRoute("/_admin/a/dashboard")({
    component: AdminDashboard,
});
