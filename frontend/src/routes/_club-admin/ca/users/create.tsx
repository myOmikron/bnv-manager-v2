import { createFileRoute } from "@tanstack/react-router";

import React from "react";
import HeadingLayout from "src/components/base/heading-layout";
import { useTranslation } from "react-i18next";
import BackButton from "src/components/base/back-button";
import { Text } from "src/components/base/text";
import Form from "src/components/base/form";
import { useForm } from "@tanstack/react-form";
import { ErrorMessage, Field, FieldGroup, Fieldset, Label, RequiredLabel } from "src/components/base/fieldset";
import { Button } from "src/components/base/button";
import { Api } from "src/api/api";
import LanguageSelect, { Lang } from "src/components/base/language-select";
import { toast } from "react-toastify";
import { Input } from "src/components/base/input";
import { CreateUserInviteResponse } from "src/api/generated";
import { Dialog, DialogActions, DialogBody, DialogDescription } from "src/components/base/dialog";
import { DialogTitle } from "@headlessui/react";
import { ClipboardDocumentListIcon } from "@heroicons/react/20/solid";

/**
 * The properties for {@link UserCreation}
 */
export type UserCreationProps = {};

/**
 * The user creation view
 */
function UserCreation(props: UserCreationProps) {
    const [t] = useTranslation();
    const [tU] = useTranslation("club-admin-create-user");

    const navigate = Route.useNavigate();

    const [newUser, setNewUser] = React.useState<CreateUserInviteResponse>();

    const form = useForm({
        defaultValues: {
            username: "",
            displayName: "",
            preferredLang: "DE" as Lang,
        },
        onSubmit: async ({ formApi, value }) => {
            const res = await Api.clubAdmin.userInvites.create({
                username: value.username,
                display_name: value.displayName,
                preferred_lang: value.preferredLang,
            });

            res.match(
                (res) => {
                    if (res.result === "Ok") {
                        toast.success(tU("toast.success"));
                        setNewUser(res.value);
                    } else {
                        if (res.error.username_in_use) {
                            formApi.setFieldMeta("username", (meta) => {
                                meta.errors.push(tU("error.username-in-use"));
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
            <BackButton href={"/ca/users"}>
                <Text>{tU("button.back-to-users")}</Text>
            </BackButton>
            <HeadingLayout className={"mt-6"} heading={tU("heading.user-creation")}>
                <Form onSubmit={form.handleSubmit} className={"max-w-lg"}>
                    <Fieldset>
                        <FieldGroup>
                            <form.Field name={"username"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.username")}</RequiredLabel>
                                        <Input
                                            autoFocus={true}
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                        />
                                        {fieldApi.state.meta.errors.map((err) => (
                                            <ErrorMessage>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </form.Field>

                            <form.Field name={"displayName"}>
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.display-name")}</RequiredLabel>
                                        <Input
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            invalid={fieldApi.state.meta.errors.length > 0}
                                        />
                                        {fieldApi.state.meta.errors.map((err) => (
                                            <ErrorMessage>{err}</ErrorMessage>
                                        ))}
                                    </Field>
                                )}
                            </form.Field>

                            <form.Field name={"preferredLang"}>
                                {(fieldApi) => (
                                    <Field>
                                        <Label>{t("label.preferred-lang")}</Label>
                                        <LanguageSelect lang={fieldApi.state.value} setLang={fieldApi.handleChange} />
                                    </Field>
                                )}
                            </form.Field>

                            <Button type={"submit"}>{t("button.create")}</Button>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </HeadingLayout>

            {newUser && (
                <Dialog open={true} onClose={() => navigate({ to: "/ca/users" })}>
                    <Form onSubmit={() => navigate({ to: "/ca/users" })}>
                        <DialogTitle>{tU("heading.dialog-user-created")}</DialogTitle>
                        <DialogDescription>{tU("description.dialog-user-created")}</DialogDescription>
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

export const Route = createFileRoute("/_club-admin/ca/users/create")({
    component: UserCreation,
});
