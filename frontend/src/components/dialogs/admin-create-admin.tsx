import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogTitle } from "src/components/base/dialog";
import Form from "src/components/base/form";
import { useForm } from "@tanstack/react-form";
import { ErrorMessage, Field, FieldGroup, Fieldset, Label, RequiredLabel } from "src/components/base/fieldset";
import { Button, PrimaryButton } from "src/components/base/button";
import { Api, UUID } from "src/api/api";
import { Input } from "src/components/base/input";
import { ClipboardDocumentListIcon } from "@heroicons/react/20/solid";
import { toast } from "react-toastify";

/**
 * The properties for {@link DialogCreateAdmin}
 */
export type DialogCreateAdminProps = {
    /** Callback for close action */
    onClose: () => void;
};

/**
 * A dialog to create an admin user
 */
export default function DialogCreateAdmin(props: DialogCreateAdminProps) {
    const [t] = useTranslation("dialog-create-admin");
    const [tg] = useTranslation();

    const [openShowInvite, setOpenShowInvite] = React.useState<UUID>();

    const form = useForm({
        defaultValues: {
            username: "",
            displayName: "",
            validDays: "7",
        },
        validators: {
            // eslint-disable-next-line
            onSubmitAsync: async ({ value }) => {
                const res = await Api.admin.invites.create({
                    username: value.username,
                    display_name: value.displayName,
                    valid_days: parseInt(value.validDays),
                    permissions: {
                        admin: true,
                        club_admin: [],
                        club_user: [],
                    },
                });

                if (res.result === "Err") {
                    return {
                        fields: {
                            username: res.error.username_already_occupied ? t("error.username-already-occupied") : null,
                            validDays: res.error.valid_days_too_small ? t("error.valid-days-too-small") : null,
                        },
                    };
                }

                setOpenShowInvite(res.value.link);
            },
        },
    });

    return (
        <Dialog open={true} onClose={props.onClose}>
            <DialogTitle>{t("heading.create-admin")}</DialogTitle>
            <DialogBody>
                <Form onSubmit={form.handleSubmit}>
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
                                    </Field>
                                )}
                            </form.Field>

                            <form.Field name={"validDays"}>
                                {(fieldApi) => (
                                    <Field>
                                        <Label>{t("label.valid-days")}</Label>
                                        <Input
                                            required={true}
                                            type={"number"}
                                            min={1}
                                            max={255}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                        />
                                    </Field>
                                )}
                            </form.Field>

                            <DialogActions>
                                <Button onClick={props.onClose} plain={true}>
                                    {tg("button.cancel")}
                                </Button>
                                <PrimaryButton type={"submit"}>{t("button.create-invite")}</PrimaryButton>
                            </DialogActions>
                        </FieldGroup>
                    </Fieldset>
                </Form>
            </DialogBody>

            {openShowInvite && (
                <Suspense>
                    <Dialog open={true} onClose={props.onClose}>
                        <DialogBody>
                            <div className={"flex gap-3"}>
                                <Input readOnly={true} defaultValue={openShowInvite} />
                                <Button
                                    outline={true}
                                    onClick={async () => {
                                        await navigator.clipboard.writeText(openShowInvite);
                                        toast.success(tg("toast.copied-to-clipboard"));
                                    }}
                                >
                                    <span className={"sr-only"}>{t("accessibility.copy")}</span>
                                    <ClipboardDocumentListIcon />
                                </Button>
                            </div>
                        </DialogBody>
                        <DialogActions>
                            <Button outline={true} onClick={props.onClose}>
                                {tg("button.close")}
                            </Button>
                        </DialogActions>
                    </Dialog>
                </Suspense>
            )}
        </Dialog>
    );
}
