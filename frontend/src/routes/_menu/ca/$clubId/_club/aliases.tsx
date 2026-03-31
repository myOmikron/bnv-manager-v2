import { createFileRoute, useRouter } from "@tanstack/react-router";
import React from "react";
import { useTranslation } from "react-i18next";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Text } from "src/components/base/text";
import { Badge } from "src/components/base/badge";
import {
    Dropdown,
    DropdownButton,
    DropdownItem,
    DropdownLabel,
    DropdownMenu,
    DropdownSection,
} from "src/components/base/dropdown";
import { CheckIcon, EllipsisVerticalIcon, TrashIcon, XMarkIcon } from "@heroicons/react/20/solid";
import { ClubAdminAliasApi, MailAliasSchema } from "src/api/api_aliases";

/**
 * The properties for {@link ClubAdminAliases}
 */
export type ClubAdminAliasesProps = {};

/**
 * ClubAdmin page for managing member aliases
 */
export default function ClubAdminAliases(props: ClubAdminAliasesProps) {
    const [t] = useTranslation("ca-aliases");
    const [tg] = useTranslation();

    const params = Route.useParams();
    const data = Route.useLoaderData();
    const router = useRouter();

    return (
        <div className={"flex flex-col gap-6"}>
            {data.length > 0 ? (
                <Table dense={true}>
                    <TableHead>
                        <TableRow>
                            <TableHeader>{t("label.address")}</TableHeader>
                            <TableHeader>{t("label.member")}</TableHeader>
                            <TableHeader>{t("label.status")}</TableHeader>
                            <TableHeader>{t("label.created-at")}</TableHeader>
                            <TableHeader className={"w-0"}>
                                <span className={"sr-only"}>{tg("accessibility.actions")}</span>
                            </TableHeader>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {data.map((alias) => (
                            <TableRow key={alias.uuid}>
                                <TableCell className={"font-medium"}>{alias.full_address}</TableCell>
                                <TableCell>{alias.account_display_name || "-"}</TableCell>
                                <TableCell>
                                    <StatusBadge status={alias.status} />
                                </TableCell>
                                <TableCell>{new Date(alias.created_at).toLocaleDateString()}</TableCell>
                                <TableCell>
                                    <Dropdown>
                                        <DropdownButton plain={true}>
                                            <EllipsisVerticalIcon />
                                        </DropdownButton>
                                        <DropdownMenu anchor={"bottom end"}>
                                            {alias.status === "Pending" && (
                                                <DropdownSection>
                                                    <DropdownItem
                                                        onClick={async () => {
                                                            await ClubAdminAliasApi.approve(params.clubId, alias.uuid);
                                                            await router.invalidate({ sync: true });
                                                        }}
                                                    >
                                                        <CheckIcon />
                                                        <DropdownLabel>{t("button.approve")}</DropdownLabel>
                                                    </DropdownItem>
                                                    <DropdownItem
                                                        onClick={async () => {
                                                            await ClubAdminAliasApi.reject(params.clubId, alias.uuid);
                                                            await router.invalidate({ sync: true });
                                                        }}
                                                    >
                                                        <XMarkIcon />
                                                        <DropdownLabel>{t("button.reject")}</DropdownLabel>
                                                    </DropdownItem>
                                                </DropdownSection>
                                            )}
                                            <DropdownSection>
                                                <DropdownItem
                                                    onClick={async () => {
                                                        await ClubAdminAliasApi.delete(params.clubId, alias.uuid);
                                                        await router.invalidate({ sync: true });
                                                    }}
                                                >
                                                    <TrashIcon />
                                                    <DropdownLabel>{t("button.delete")}</DropdownLabel>
                                                </DropdownItem>
                                            </DropdownSection>
                                        </DropdownMenu>
                                    </Dropdown>
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            ) : (
                <Text>{t("label.no-aliases")}</Text>
            )}
        </div>
    );
}

/**
 * Properties for the StatusBadge component
 */
type StatusBadgeProps = {
    /** The alias status */
    status: MailAliasSchema["status"];
};

/**
 * Displays a colored badge for the alias status
 *
 * @param props the component props
 * @param props.status the alias status
 * @returns the badge element
 */
function StatusBadge({ status }: StatusBadgeProps) {
    const [t] = useTranslation("ca-aliases");

    const color = status === "Approved" ? "green" : status === "Pending" ? "yellow" : "red";
    return <Badge color={color}>{t(`status.${status}`)}</Badge>;
}

export const Route = createFileRoute("/_menu/ca/$clubId/_club/aliases")({
    component: ClubAdminAliases,
    loader: async ({ params }) => await ClubAdminAliasApi.getAll(params.clubId),
});
