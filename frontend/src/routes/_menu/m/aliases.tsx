import { createFileRoute, useRouter } from "@tanstack/react-router";
import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import HeadingLayout from "src/components/base/heading-layout";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Text } from "src/components/base/text";
import { Badge } from "src/components/base/badge";
import { Button } from "src/components/base/button";
import { PlusIcon, TrashIcon } from "@heroicons/react/20/solid";
import { Dialog, DialogActions, DialogBody, DialogDescription, DialogTitle } from "src/components/base/dialog";
import { DomainOptionSchema, MailAliasSchema, MemberAliasApi } from "src/api/api_aliases";
import ProposeAliasDialog from "src/components/dialogs/m-propose-alias";

/**
 * The properties for {@link MemberAliases}
 */
export type MemberAliasesProps = {};

/**
 * Page for members to view and manage their email aliases
 */
export default function MemberAliases(props: MemberAliasesProps) {
    const [t] = useTranslation("m-aliases");
    const [tg] = useTranslation();

    const data = Route.useLoaderData();
    const router = useRouter();

    const [openPropose, setOpenPropose] = React.useState(false);
    const [deleteAlias, setDeleteAlias] = React.useState<MailAliasSchema>();

    return (
        <HeadingLayout
            heading={t("heading.my-aliases")}
            headingChildren={
                <Button outline={true} onClick={() => setOpenPropose(true)}>
                    <PlusIcon />
                    <span>{t("button.propose-alias")}</span>
                </Button>
            }
        >
            {data.aliases.length > 0 ? (
                <Table dense={true}>
                    <TableHead>
                        <TableRow>
                            <TableHeader>{t("label.address")}</TableHeader>
                            <TableHeader>{t("label.status")}</TableHeader>
                            <TableHeader>{t("label.created-at")}</TableHeader>
                            <TableHeader className={"w-0"}>
                                <span className={"sr-only"}>{tg("accessibility.actions")}</span>
                            </TableHeader>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {data.aliases.map((alias) => (
                            <TableRow key={alias.uuid}>
                                <TableCell className={"font-medium"}>{alias.full_address}</TableCell>
                                <TableCell>
                                    <StatusBadge status={alias.status} />
                                </TableCell>
                                <TableCell>{new Date(alias.created_at).toLocaleDateString()}</TableCell>
                                <TableCell>
                                    <Button plain={true} onClick={() => setDeleteAlias(alias)}>
                                        <TrashIcon />
                                    </Button>
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            ) : (
                <Text>{t("label.no-aliases")}</Text>
            )}

            <Suspense>
                <ProposeAliasDialog
                    open={openPropose}
                    domains={data.domains}
                    onClose={() => setOpenPropose(false)}
                    onCreate={async () => {
                        setOpenPropose(false);
                        await router.invalidate({ sync: true });
                    }}
                />
            </Suspense>

            <Suspense>
                <Dialog open={!!deleteAlias} onClose={() => setDeleteAlias(undefined)}>
                    <DialogTitle>{t("confirm.delete-title")}</DialogTitle>
                    <DialogBody>
                        <DialogDescription>
                            {t("confirm.delete-description", { address: deleteAlias?.full_address })}
                        </DialogDescription>
                    </DialogBody>
                    <DialogActions>
                        <Button onClick={() => setDeleteAlias(undefined)} plain={true}>
                            {tg("button.cancel")}
                        </Button>
                        <Button
                            color={"red"}
                            onClick={async () => {
                                if (deleteAlias) {
                                    await MemberAliasApi.delete(deleteAlias.uuid);
                                    setDeleteAlias(undefined);
                                    await router.invalidate({ sync: true });
                                }
                            }}
                        >
                            {t("button.delete")}
                        </Button>
                    </DialogActions>
                </Dialog>
            </Suspense>
        </HeadingLayout>
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
    const [t] = useTranslation("m-aliases");

    const color = status === "Approved" ? "green" : status === "Pending" ? "yellow" : "red";
    return <Badge color={color}>{t(`status.${status}`)}</Badge>;
}

/**
 * Loader data type
 */
type LoaderData = {
    /** The aliases */
    aliases: MailAliasSchema[];
    /** Available domains */
    domains: DomainOptionSchema[];
};

export const Route = createFileRoute("/_menu/m/aliases")({
    component: MemberAliases,
    loader: async (): Promise<LoaderData> => {
        const [aliases, domains] = await Promise.all([MemberAliasApi.getAll(), MemberAliasApi.getDomains()]);
        return { aliases, domains };
    },
});
