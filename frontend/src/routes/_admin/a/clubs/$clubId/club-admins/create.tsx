import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import HeadingLayout from "src/components/base/heading-layout";
import { useTranslation } from "react-i18next";
import BackButton from "src/components/base/back-button";
import { Text } from "src/components/base/text";
import Form from "src/components/base/form";
import { ErrorMessage, Field, FieldGroup, Fieldset, Label, RequiredLabel } from "src/components/base/fieldset";
import { useForm } from "@tanstack/react-form";
import { Button } from "src/components/base/button";
import { Api } from "src/api/api";
import { Input } from "src/components/base/input";
import { toast } from "react-toastify";
import LanguageSelect, { Lang } from "src/components/base/language-select";
import { Dialog, DialogActions, DialogBody, DialogDescription, DialogTitle } from "src/components/base/dialog";
import { CreateUserInviteResponse } from "src/api/generated";
import { ClipboardDocumentListIcon } from "@heroicons/react/20/solid";

/**
 * The properties for {@link CreateClubAdmin}
 */
export type CreateClubAdminProps = {};

/**
 * Create a new club admin
 */
export default function CreateClubAdmin(props: CreateClubAdminProps) {
    const [t] = useTranslation();
    const [tC] = useTranslation("admin-create-club-admin");

    const { clubId } = Route.useParams();
    const navigate = Route.useNavigate();

    const [newUser, setNewUser] = React.useState<CreateUserInviteResponse>();

    const createForm = useForm({
        defaultValues: {
            username: "",
            displayName: "",
            preferred_lang: "DE" as Lang,
        },
        onSubmit: async ({ value, formApi }) => {
            const username = value.username;
            const display_name = value.displayName;
            const preferred_lang = value.preferred_lang;

            const res = await Api.admin.invites.create({
                username,
                display_name,
                role: { role: "ClubAdmin", club: clubId },
                preferred_lang,
            });

            res.match(
                (res) => {
                    if (res.result === "Ok") {
                        setNewUser(res.value);
                    } else {
                        if (res.error.username_in_use) {
                            formApi.setFieldMeta("username", (meta) => {
                                meta.errors.push(tC("error.username-in-use"));
                                return meta;
                            });
                        }
                    }
                },
                (err) => toast.error(err.message),
            );
        },
    });

    return (
        <>
            <BackButton href={"/a/clubs/$clubId/general"} params={{ clubId }}>
                <Text className={"font-light"}>{tC("button.back-to-club")}</Text>
            </BackButton>

            <HeadingLayout className={"mt-6"} heading={tC("heading.create-club-admin")}>
                <Form onSubmit={createForm.handleSubmit} className={"max-w-lg"}>
                    <Fieldset>
                        <FieldGroup>
                            <createForm.Field name={"username"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.username")}</RequiredLabel>
                                        <Input
                                            autoFocus={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.setValue(e.target.value)}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                            required={true}
                                        />
                                        {fieldApi.state.meta.errors.map((err) => (
                                            <ErrorMessage>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </createForm.Field>

                            <createForm.Field name={"displayName"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.display-name")}</RequiredLabel>
                                        <Input
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.setValue(e.target.value)}
                                            required={true}
                                        />
                                    </Field>
                                )}
                            </createForm.Field>

                            <createForm.Field name={"preferred_lang"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.preferred-lang")}</RequiredLabel>
                                        <LanguageSelect lang={fieldApi.state.value} setLang={fieldApi.handleChange} />
                                    </Field>
                                )}
                            </createForm.Field>

                            <Button type={"submit"}>{t("button.create")}</Button>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </HeadingLayout>

            {newUser && (
                <Dialog open={true} onClose={() => setNewUser(undefined)}>
                    <Form onSubmit={() => navigate({ to: "/a/clubs/$clubId/general", params: { clubId } })}>
                        <DialogTitle>{tC("heading.dialog-user-created")}</DialogTitle>
                        <DialogDescription>{tC("description.dialog-user-created")}</DialogDescription>
                        <DialogBody>
                            <FieldGroup>
                                <Field>
                                    <Label>{t("label.invite-link")}</Label>
                                    <div className={"mt-3 flex gap-3"}>
                                        <Input autoFocus={true} defaultValue={newUser.link} readOnly={true} />
                                        <Button
                                            plain={true}
                                            onClick={async () => {
                                                await navigator.clipboard.writeText(newUser.link);
                                                toast.success(t("toast.copied-to-clipboard"));
                                            }}
                                        >
                                            <span className={"sr-only"}>{t("accessibility.copy")}</span>
                                            <ClipboardDocumentListIcon />
                                        </Button>
                                    </div>
                                </Field>
                            </FieldGroup>
                        </DialogBody>
                        <DialogActions>
                            <Button type={"submit"}>{t("button.finish")}</Button>
                        </DialogActions>
                    </Form>
                </Dialog>
            )}
        </>
    );
}

export const Route = createFileRoute("/_admin/a/clubs/$clubId/club-admins/create")({
    component: CreateClubAdmin,
});
