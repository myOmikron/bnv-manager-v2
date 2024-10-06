import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import BackButton from "src/components/base/back-button";
import { Text } from "src/components/base/text";
import { useTranslation } from "react-i18next";
import HeadingLayout from "src/components/base/heading-layout";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Api } from "src/api/api";
import { Button } from "src/components/base/button";
import { toast } from "react-toastify";
import { SimpleUser } from "src/api/generated";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { ArrowDownOnSquareStackIcon, EllipsisVerticalIcon } from "@heroicons/react/24/outline";
import { Field, Label } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";

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
                        <Button outline={true}>
                            <span>{tU("button.export-csv")}</span>
                            <ArrowDownOnSquareStackIcon className={"!size-5"} />
                        </Button>
                        <Button outline={true}>
                            <span>{tU("button.export-json")}</span>
                            <ArrowDownOnSquareStackIcon className={"!size-5"} />
                        </Button>
                    </div>
                </div>

                <Table>
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
                                <TableCell>{}</TableCell>
                                <TableCell>
                                    <Dropdown>
                                        <DropdownButton plain={true}>
                                            <EllipsisVerticalIcon />
                                        </DropdownButton>

                                        <DropdownMenu anchor={"bottom end"}>
                                            <DropdownItem>
                                                <DropdownLabel>{tU("button.create-reset-link")}</DropdownLabel>
                                            </DropdownItem>
                                            <DropdownItem>
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
        </>
    );
}

export const Route = createFileRoute("/_club-admin/ca/users/")({
    component: UserOverview,
});
