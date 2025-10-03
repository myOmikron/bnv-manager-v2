import { createFileRoute } from "@tanstack/react-router";
import React from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { EllipsisVerticalIcon, LinkIcon } from "@heroicons/react/20/solid";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { toast } from "react-toastify";

/**
 * The properties for {@link InvitedClubMembers}
 */
export type InvitedClubMembersProps = {};

/**
 * Overview over club members
 */
export default function InvitedClubMembers(props: InvitedClubMembersProps) {
    const [t] = useTranslation("ca-club-view");
    const [tg] = useTranslation();

    const data = Route.useLoaderData();

    return (
        <div className={"flex flex-col gap-6"}>
            <Table dense={true}>
                <TableHead>
                    <TableRow>
                        <TableHeader>{t("label.username")}</TableHeader>
                        <TableHeader>{t("label.display-name")}</TableHeader>
                        <TableHeader>{t("label.expires-at")}</TableHeader>
                        <TableHeader className={"w-0"}>
                            <span className={"sr-only"}>{tg("accessibility.actions")}</span>
                        </TableHeader>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {data.map((item) => (
                        <TableRow key={item.uuid}>
                            <TableCell>{item.username}</TableCell>
                            <TableCell>{item.display_name}</TableCell>
                            <TableCell>{new Date(item.expires_at).toLocaleDateString("de-de")}</TableCell>
                            <TableCell>
                                <Dropdown>
                                    <DropdownButton plain={true}>
                                        <EllipsisVerticalIcon />
                                    </DropdownButton>
                                    <DropdownMenu anchor={"bottom end"}>
                                        <DropdownItem
                                            onClick={async () => {
                                                await navigator.clipboard.writeText(item.link);
                                                toast.success(tg("toast.copied-to-clipboard"));
                                            }}
                                        >
                                            <LinkIcon />
                                            <DropdownLabel>{t("button.copy-invite-link")}</DropdownLabel>
                                        </DropdownItem>
                                    </DropdownMenu>
                                </Dropdown>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </div>
    );
}

export const Route = createFileRoute("/_menu/ca/$clubId/_club/invited")({
    component: InvitedClubMembers,
    loader: async ({ params }) => await Api.clubAdmins.club.getInvitedMembers(params.clubId),
});
