import { createFileRoute, useRouter } from "@tanstack/react-router";

import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import TablePagination from "src/components/table-pagination";
import { Text } from "src/components/base/text";
import { Button } from "src/components/base/button";
import { EllipsisVerticalIcon, LinkIcon, PlusIcon } from "@heroicons/react/20/solid";
import ClubAdminCreateMemberInviteDialog from "src/components/dialogs/ca-create-member";
import { Subheading } from "src/components/base/heading";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { toast } from "react-toastify";

/**
 * The properties for {@link ClubMembers}
 */
export type ClubMembersProps = {};

/**
 * Overview over club members
 */
export default function ClubMembers(props: ClubMembersProps) {
    const [t] = useTranslation("ca-club-view");
    const [tg] = useTranslation();

    const params = Route.useParams();
    const router = useRouter();
    const data = Route.useLoaderData();
    const search = Route.useSearch();

    const [openCreateMember, setOpenCreateMember] = React.useState(false);

    return (
        <div className={"flex flex-col gap-6"}>
            <div className={"flex justify-end"}>
                <Button outline={true} onClick={() => setOpenCreateMember(true)}>
                    <PlusIcon />
                    <span>{t("button.create-member")}</span>
                </Button>
            </div>
            {data.invites.length > 0 && (
                <>
                    <Subheading>{t("heading.invited")}</Subheading>
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
                            {data.invites.map((item) => (
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
                </>
            )}

            <Subheading className={"mt-8"}>{t("heading.members")}</Subheading>
            {data.members.total > 0 ? (
                <>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableHeader>{t("label.username")}</TableHeader>
                                <TableHeader>{t("label.display-name")}</TableHeader>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {data.members.items.map((item) => (
                                <TableRow key={item.uuid}>
                                    <TableCell>{item.username}</TableCell>
                                    <TableCell>{item.display_name}</TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                    <TablePagination
                        href={"/ca/$clubId/members"}
                        params={params}
                        maxPages={Math.ceil(data.members.total / LIMIT)}
                        currentPage={search.page}
                        getSearchParams={(newPage) => ({ page: newPage, search: search.search })}
                    />
                </>
            ) : (
                <Text>{t("label.no-members")}</Text>
            )}

            <Suspense>
                {openCreateMember && (
                    <ClubAdminCreateMemberInviteDialog
                        club={params.clubId}
                        onClose={() => setOpenCreateMember(false)}
                        onCreate={async () => {
                            setOpenCreateMember(false);
                            await router.invalidate({ sync: true });
                        }}
                    />
                )}
            </Suspense>
        </div>
    );
}

const LIMIT = 20;

/**
 * Parameter for this endpoint
 */
type SearchParams = {
    /** Current page that should be displayed */
    page: number;
    /** Search for a user */
    search?: string;
};

export const Route = createFileRoute("/_menu/ca/$clubId/_club/members")({
    component: ClubMembers,
    validateSearch: (search: Record<string, unknown>): SearchParams => {
        const page = Number(search?.page ?? 1);

        return {
            page: page <= 0 ? 1 : page,
            search: search?.search as string | undefined,
        };
    },
    loaderDeps: ({ search: { page, search } }) => ({ page, search }),
    loader: async ({ params, deps }) => ({
        members: await Api.clubAdmins.club.getMembers({
            club_uuid: params.clubId,
            limit: LIMIT,
            offset: (deps.page - 1) * LIMIT,
            search: deps.search,
        }),
        invites: await Api.clubAdmins.club.getInvitedMembers(params.clubId),
    }),
});
