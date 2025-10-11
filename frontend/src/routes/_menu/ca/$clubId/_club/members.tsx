import { createFileRoute } from "@tanstack/react-router";
import React from "react";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import TablePagination from "src/components/table-pagination";
import { Text } from "src/components/base/text";
import {
    Dropdown,
    DropdownButton,
    DropdownHeading,
    DropdownItem,
    DropdownLabel,
    DropdownMenu,
    DropdownSection,
} from "src/components/base/dropdown";
import { ArrowDownTrayIcon, EllipsisVerticalIcon, TrashIcon } from "@heroicons/react/20/solid";
import { Button } from "src/components/base/button";
import { Field, FieldGroup, Fieldset, Label } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";
import { downloadFile } from "src/utils/downloader";
import Form from "src/components/base/form";
import { useForm } from "@tanstack/react-form";

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
    const data = Route.useLoaderData();
    const search = Route.useSearch();
    const navigate = Route.useNavigate();

    const form = useForm({
        defaultValues: {
            search: search.search,
        },
    });

    return (
        <div className={"flex flex-col gap-6"}>
            {data.total > 0 ? (
                <>
                    <div className={"flex justify-between"}>
                        <Form onSubmit={form.handleSubmit}>
                            <Fieldset>
                                <FieldGroup>
                                    <form.Field
                                        name={"search"}
                                        validators={{
                                            onChangeAsync: async ({ fieldApi }) => {
                                                await navigate({
                                                    to: "/ca/$clubId/members",
                                                    search: {
                                                        page: 1,
                                                        search: fieldApi.state.value,
                                                    },
                                                });
                                            },
                                            onChangeAsyncDebounceMs: 500,
                                        }}
                                    >
                                        {(fieldApi) => (
                                            <Field>
                                                <Label>{t("label.search")}</Label>
                                                <Input
                                                    required={true}
                                                    value={fieldApi.state.value}
                                                    onChange={(e) => fieldApi.handleChange(e.target.value)}
                                                />
                                            </Field>
                                        )}
                                    </form.Field>
                                </FieldGroup>
                            </Fieldset>
                        </Form>

                        <div className={"flex gap-3"}>
                            <Button
                                className={"h-fit w-fit items-center self-end"}
                                outline={true}
                                onClick={async () => {
                                    const page = await Api.clubAdmins.club.getMembers({
                                        club_uuid: params.clubId,
                                        search: search.search,
                                        limit: 100_000,
                                        offset: 0,
                                    });
                                    downloadFile("members.json", JSON.stringify(page.items));
                                }}
                            >
                                <ArrowDownTrayIcon />
                                <span>{t("label.download-as-json")}</span>
                            </Button>
                            <Button
                                className={"h-fit w-fit items-center self-end"}
                                outline={true}
                                onClick={async () => {
                                    const page = await Api.clubAdmins.club.getMembers({
                                        club_uuid: params.clubId,
                                        search: search.search,
                                        limit: 100_000,
                                        offset: 0,
                                    });
                                    downloadFile(
                                        "members.csv",
                                        "Username,Display name,Email\n" +
                                            page.items
                                                .map(
                                                    (member) =>
                                                        `${member.username},${member.display_name},${member.email}`,
                                                )
                                                .join("\n") +
                                            "\n",
                                    );
                                }}
                            >
                                <ArrowDownTrayIcon />
                                <span>{t("label.download-as-csv")}</span>
                            </Button>
                        </div>
                    </div>
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
                            {data.items.map((item) => (
                                <TableRow key={item.uuid}>
                                    <TableCell>{item.username}</TableCell>
                                    <TableCell>{item.display_name}</TableCell>
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
                                                        <DropdownLabel>{t("label.delete-member")}</DropdownLabel>
                                                    </DropdownItem>
                                                </DropdownSection>
                                            </DropdownMenu>
                                        </Dropdown>
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                    <TablePagination
                        href={"/ca/$clubId/members"}
                        params={params}
                        maxPages={Math.ceil(data.total / LIMIT)}
                        currentPage={search.page}
                        getSearchParams={(newPage) => ({ page: newPage, search: search.search })}
                    />
                </>
            ) : (
                <Text>{t("label.no-members")}</Text>
            )}
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
    search: string;
};

export const Route = createFileRoute("/_menu/ca/$clubId/_club/members")({
    component: ClubMembers,
    validateSearch: (search: Record<string, unknown>): SearchParams => {
        const page = Number(search?.page ?? 1);

        return {
            page: page <= 0 ? 1 : page,
            search: search?.search as string | "",
        };
    },
    loaderDeps: ({ search: { page, search } }) => ({ page, search }),
    loader: async ({ params, deps }) =>
        await Api.clubAdmins.club.getMembers({
            club_uuid: params.clubId,
            limit: LIMIT,
            offset: (deps.page - 1) * LIMIT,
            search: deps.search,
        }),
});
