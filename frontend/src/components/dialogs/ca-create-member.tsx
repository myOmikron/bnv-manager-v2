import React, { Suspense } from "react";
import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogTitle } from "src/components/base/dialog";
import { Api, UUID } from "src/api/api";
import { Button, PrimaryButton } from "src/components/base/button";
import { useForm } from "@tanstack/react-form";
import Form from "src/components/base/form";
import { Description, ErrorMessage, Field, FieldGroup, Fieldset, RequiredLabel } from "src/components/base/fieldset";
import { Input } from "src/components/base/input";
import { toast } from "react-toastify";
import { ClipboardDocumentListIcon } from "@heroicons/react/20/solid";
import { Club } from "src/api/generated/club-admin";

/**
 * The properties for {@link ClubAdminCreateMemberInviteDialog}
 */
export type ClubAdminCreateMemberInviteDialogProps = {
    /** The club to create the invite for */
    club: Club;
    /** Should the dialog be open */
    open: boolean;
    /** Callback for close action */
    onClose: () => void;
    /** Callback for creation of the new invite */
    onCreate: () => void;
};

/**
 * Dialog for creating a member invite
 */
export default function ClubAdminCreateMemberInviteDialog(props: ClubAdminCreateMemberInviteDialogProps) {
    const [t] = useTranslation("dialog-ca-create-member-invite");
    const [tg] = useTranslation();

    const [openShowInvite, setOpenShowInvite] = React.useState<UUID>();

    const form = useForm({
        defaultValues: {
            username: "",
            displayName: "",
            validDays: "7",
            email: "",
        },
        validators: {
            onSubmitAsync: async ({ value }) => {
                const res = await Api.clubAdmins.invites.create(props.club.uuid, {
                    username: value.username,
                    display_name: value.displayName,
                    valid_days: parseInt(value.validDays),
                    email: value.email,
                });

                if (res.result === "Err") {
                    return {
                        fields: {
                            username: res.error.username_already_occupied ? t("error.username-already-occupied") : null,
                        },
                    };
                }

                setOpenShowInvite(res.value.link);
            },
        },
    });

    return (
        <Dialog open={props.open} onClose={props.onClose}>
            <DialogTitle>{t("heading.invite-new-member")}</DialogTitle>
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

                            <form.Field
                                name={"email"}
                                validators={{
                                    onBlur: ({ value }) => {
                                        if (!value.endsWith(props.club.primary_domain))
                                            return t("error.invalid-email-domain", {
                                                domain: props.club.primary_domain,
                                            });
                                    },
                                }}
                            >
                                {(fieldApi) => (
                                    <Field>
                                        <RequiredLabel>{t("label.email")}</RequiredLabel>
                                        <Input
                                            type={"email"}
                                            required={true}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.handleChange(e.target.value)}
                                            onBlur={() => fieldApi.handleBlur()}
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
                                        <RequiredLabel>{t("label.valid-days")}</RequiredLabel>
                                        <Description>{t("description.valid-days")}</Description>
                                        <Input
                                            required={true}
                                            type={"number"}
                                            min={1}
                                            max={14}
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

            <Suspense>
                <Dialog
                    open={!!openShowInvite}
                    onClose={() => {
                        form.reset();
                        props.onClose();
                    }}
                >
                    <DialogTitle>{t("heading.invite-created")}</DialogTitle>
                    <DialogBody>
                        <div className={"flex gap-3"}>
                            <Input readOnly={true} defaultValue={openShowInvite} />
                            <Button
                                outline={true}
                                onClick={async () => {
                                    openShowInvite && (await navigator.clipboard.writeText(openShowInvite));
                                    toast.success(tg("toast.copied-to-clipboard"));
                                }}
                            >
                                <span className={"sr-only"}>{t("accessibility.copy")}</span>
                                <ClipboardDocumentListIcon />
                            </Button>
                        </div>
                    </DialogBody>
                    <DialogActions>
                        <Button outline={true} onClick={props.onCreate}>
                            {tg("button.close")}
                        </Button>
                    </DialogActions>
                </Dialog>
            </Suspense>
        </Dialog>
    );
}
