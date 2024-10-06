import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import BackButton from "src/components/base/back-button";
import { useTranslation } from "react-i18next";
import { Text } from "src/components/base/text";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import { FullClub } from "src/api/generated";
import Stats from "src/components/base/stats";
import { Heading } from "src/components/base/heading";
import { Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu } from "src/components/base/dropdown";
import { EllipsisVerticalIcon } from "@heroicons/react/20/solid";
import ConfirmDialog from "src/components/confirm-dialog";
import { Dialog, DialogActions, DialogBody, DialogTitle } from "src/components/base/dialog";
import Form from "src/components/base/form";
import { Button } from "src/components/base/button";
import { ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";

/**
 * The properties for {@link ClubView}
 */
export type ClubViewProps = {};

/**
 * The overview of a single club
 */
function ClubView(props: ClubViewProps) {
    const [t] = useTranslation();
    const [tC] = useTranslation("club-view");

    const { clubId } = Route.useParams();
    const navigate = Route.useNavigate();

    const [club, setClub] = React.useState<FullClub>();

    const [openDeleteClub, setOpenDeleteClub] = React.useState(false);
    const [openRenameClub, setOpenRenameClub] = React.useState<{
        /** New name of the club */
        newName: string;
        /** List of possible errors */
        errors: Array<string>;
    }>();

    /**
     * Retrieve a club
     */
    const getClub = async () => {
        const res = await Api.admin.clubs.get(clubId);

        res.match(
            (c) => setClub(c),
            (err) => toast.error(err.message),
        );
    };

    /**
     * Delete the current club
     */
    const deleteClub = async () => {
        const res = await Api.admin.clubs.delete(clubId);
        if (res.isOk) {
            toast.success(tC("toast.club-deleted"));
        } else {
            toast.error(res.err.message);
        }
    };

    /**
     * Rename the club
     *
     * @param name The new name
     */
    const renameClub = async (name: string) => {
        const res = await Api.admin.clubs.update(clubId, { name });

        res.match(
            (res) => {
                if (res.result === "Ok") {
                    getClub().then(() => setOpenRenameClub(undefined));
                } else {
                    if (res.error.name_in_use) {
                        setOpenRenameClub({
                            errors: [tC("errors.club-name-in-use")],
                            newName: openRenameClub?.newName || "",
                        });
                    }
                }
            },
            (err) => toast.error(err.message),
        );
    };

    useEffect(() => {
        getClub().then();
    }, [clubId]);

    if (!club) {
        return undefined;
    }

    return (
        <>
            <BackButton href={"/a/dashboard"}>
                <Text className={"!text-sm font-normal"}>{t("button.back")}</Text>
            </BackButton>
            <div className={"mt-6 flex w-full flex-col gap-6"}>
                <div className={"flex justify-between gap-6"}>
                    <Heading>{tC("heading.club-overview", { club: club.name })}</Heading>
                    <Dropdown>
                        <DropdownButton plain={true}>
                            <EllipsisVerticalIcon />
                            <span className={"sr-only"}>{t("accessibility.actions")}</span>
                        </DropdownButton>
                        <DropdownMenu anchor={"bottom end"}>
                            <DropdownItem href={"/a/clubs/$clubId/club-admins/create"} params={{ clubId: club.uuid }}>
                                <DropdownLabel>{tC("button.add-club-admin")}</DropdownLabel>
                            </DropdownItem>
                            <DropdownItem onClick={() => setOpenRenameClub({ errors: [], newName: "" })}>
                                <DropdownLabel>{tC("button.rename-club")}</DropdownLabel>
                            </DropdownItem>
                            <DropdownItem onClick={() => setOpenDeleteClub(true)}>
                                <DropdownLabel>{tC("button.delete-club")}</DropdownLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                </div>

                <div className={"grid grid-cols-1 gap-6 sm:grid-cols-3"}>
                    <Stats label={tC("label.user-count")} value={club.user_count} />
                </div>
            </div>

            {openDeleteClub && (
                <ConfirmDialog
                    title={tC("heading.delete-club", { name: club.name })}
                    description={tC("description.delete-club")}
                    onConfirm={() => deleteClub().then(() => navigate({ to: "/a/dashboard" }))}
                    onCancel={() => setOpenDeleteClub(false)}
                />
            )}

            {openRenameClub !== undefined && (
                <Dialog open={true} onClose={() => setOpenRenameClub(undefined)}>
                    <Form onSubmit={() => openRenameClub && renameClub(openRenameClub.newName)}>
                        <DialogTitle>{tC("heading.rename-club")}</DialogTitle>
                        <DialogBody>
                            <Fieldset>
                                <FieldGroup>
                                    <Field>
                                        <RequiredLabel>{tC("label.new-club-name")}</RequiredLabel>
                                        <Input
                                            autoFocus={true}
                                            required={true}
                                            value={openRenameClub.newName}
                                            invalid={openRenameClub.errors.length > 0}
                                            onChange={(e) => {
                                                openRenameClub &&
                                                    setOpenRenameClub({
                                                        errors: [],
                                                        newName: e.target.value,
                                                    });
                                            }}
                                        />
                                        {openRenameClub.errors.map((err) => (
                                            <ErrorMessage>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                </FieldGroup>
                            </Fieldset>
                        </DialogBody>
                        <DialogActions>
                            <Button plain={true} onClick={() => setOpenRenameClub(undefined)}>
                                {t("button.cancel")}
                            </Button>
                            <Button type={"submit"}>{tC("button.rename-club")}</Button>
                        </DialogActions>
                    </Form>
                </Dialog>
            )}
        </>
    );
}

export const Route = createFileRoute("/_admin/a/clubs/$clubId/general/")({
    component: ClubView,
});
