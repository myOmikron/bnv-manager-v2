import { createFileRoute, useRouter } from "@tanstack/react-router";

import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import HeadingLayout from "src/components/base/heading-layout";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { PrimaryButton } from "src/components/base/button";
import { EllipsisVerticalIcon, PlusIcon, TrashIcon } from "@heroicons/react/20/solid";
import AdminCreateClubDialog from "src/components/dialogs/admin-create-club";
import {
    Dropdown,
    DropdownButton,
    DropdownHeading,
    DropdownItem,
    DropdownLabel,
    DropdownMenu,
    DropdownSection
} from "src/components/base/dropdown";

/**
 * The properties for {@link AdminClubOverview}
 */
export type AdminClubOverviewProps = {};

/**
 * The overview of clubs for admins
 */
function AdminClubOverview(props: AdminClubOverviewProps) {
    const [t] = useTranslation("admin-clubs");
    const [tg] = useTranslation();

    const router = useRouter();
    const clubs = Route.useLoaderData();

    const [openCreateClub, setOpenCreateClub] = React.useState(false);

    return (
        <HeadingLayout
            heading={t("heading.clubs-overview")}
            headingChildren={
                <PrimaryButton onClick={() => setOpenCreateClub(true)}>
                    <PlusIcon />
                    <span>{t("button.create-club")}</span>
                </PrimaryButton>
            }
        >
            <Table>
                <TableHead>
                    <TableRow>
                        <TableHeader>{t("label.club-name")}</TableHeader>
                        <TableHeader>{t("label.created-at")}</TableHeader>
                        <TableHeader className={"w-0"}>
                            <span className={"sr-only"}>{tg("accessibility.actions")}</span>
                        </TableHeader>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {clubs.map((club) => (
                        <TableRow key={club.uuid}>
                            <TableCell>{club.name}</TableCell>
                            <TableCell>{new Date(club.created_at).toLocaleDateString("de-de")}</TableCell>
                            <TableCell>
                                <Dropdown>
                                    <DropdownButton plain={true}>
                                        <EllipsisVerticalIcon />
                                    </DropdownButton>
                                    <DropdownMenu anchor={"bottom end"}>
                                        <DropdownSection>
                                            <DropdownHeading>{tg("heading.danger-zone")}</DropdownHeading>

                                            <DropdownItem>
                                                <TrashIcon />
                                                <DropdownLabel>{t("button.delete-club")}</DropdownLabel>
                                            </DropdownItem>
                                        </DropdownSection>
                                    </DropdownMenu>
                                </Dropdown>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>

            {openCreateClub && (
                <Suspense>
                    <AdminCreateClubDialog
                        onClose={() => setOpenCreateClub(false)}
                        onCreate={() => {
                            setOpenCreateClub(false);
                            router.invalidate({ sync: true });
                        }}
                    />
                </Suspense>
            )}
        </HeadingLayout>
    );
}

export const Route = createFileRoute("/_menu/a/clubs/")({
    component: AdminClubOverview,
    // eslint-disable-next-line
    loader: async () => await Api.admin.clubs.getAll(),
});
